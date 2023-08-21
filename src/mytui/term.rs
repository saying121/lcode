use std::{sync::{Arc, RwLock}, io::{BufWriter, Write}};

use bytes::Bytes;
use miette::Result;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use tokio::{sync::mpsc::{Sender, channel}, task::spawn_blocking};

#[derive(Debug, Clone)]
pub struct Size {
    cols: u16,
    rows: u16,
}

pub struct PtyPane {
    parser: Arc<RwLock<vt100::Parser>>,
    sender: Sender<Bytes>,
}

impl PtyPane {
    pub fn new(size: Size, cmd: CommandBuilder) -> Result<Self> {
        let pty_system = native_pty_system();
        let pty_pair = pty_system
            .openpty(PtySize {
                rows: size.rows - 4,
                cols: size.cols - 4,
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();
        let parser = Arc::new(RwLock::new(vt100::Parser::new(
            size.rows - 4,
            size.cols - 4,
            0,
        )));

        spawn_blocking(move || {
            let mut child = pty_pair
                .slave
                .spawn_command(cmd)
                .unwrap();
            let _ = child.wait();
            drop(pty_pair.slave);
        });

        {
            let mut reader = pty_pair
                .master
                .try_clone_reader()
                .unwrap();
            let parser = parser.clone();
            tokio::spawn(async move {
                let mut processed_buf = Vec::new();
                let mut buf = [0u8; 8192];

                loop {
                    let size = reader.read(&mut buf).unwrap();
                    if size == 0 {
                        break;
                    }
                    if size > 0 {
                        processed_buf.extend_from_slice(&buf[..size]);
                        let mut parser = parser.write().unwrap();
                        parser.process(&processed_buf);

                        // Clear the processed portion of the buffer
                        processed_buf.clear();
                    }
                }
            });
        }

        let (tx, mut rx) = channel::<Bytes>(32);

        {
            let mut writer = BufWriter::new(
                pty_pair
                    .master
                    .take_writer()
                    .unwrap(),
            );
            // Drop writer on purpose
            tokio::spawn(async move {
                while let Some(bytes) = rx.recv().await {
                    writer.write_all(&bytes).unwrap();
                    writer.flush().unwrap();
                }
                drop(pty_pair.master);
            });
        }

        Ok(Self { parser, sender: tx })
    }
}
