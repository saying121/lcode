use leetcode_api::{
    glob_leetcode,
    leetcode::{question::qs_detail::Question, resps::run_res::*, IdSlug},
};
use tracing::error;

use crate::{app::inner::App, mytui::myevent::UserEvent};

impl<'app_lf> App<'app_lf> {
    pub fn get_qs_detail(&self, idslug: IdSlug, force: bool) {
        let eve_tx = self.events.tx.clone();
        tokio::spawn(async move {
            let qs = glob_leetcode()
                .await
                .get_qs_detail(idslug, force)
                .await
                .unwrap_or_else(Question::new_with_info);
            eve_tx
                .send(UserEvent::GetQsDone(Box::new(qs)))
                .expect("get_qs_detail send failed");
        });
    }
    pub async fn get_qs_done(&mut self, qs: Question) {
        match self.get_code(&qs).await {
            // if error, don't update question info
            Ok(()) => self.cur_qs = qs,
            Err(err) => error!("{}", err),
        };
        self.render();
    }
    pub fn menu_button_trig(&mut self) -> bool {
        self.edit.button.active_but();
        match self.edit.button.selected {
            0 => self.test_code(),
            1 => self.submit_code(),
            _ => false,
        }
    }
    pub fn submit_code(&mut self) -> bool {
        let id: u32 = self
            .cur_qs
            .question_id
            .parse()
            .unwrap_or_default();

        // avoid repeated requests
        if self.edit.button.submitting {
            return false;
        }

        self.edit.button.done();
        let eve_tx = self.events.tx.clone();
        tokio::spawn(async move {
            // min id is 1
            let runres = if id > 0 {
                match glob_leetcode()
                    .await
                    .submit_code(IdSlug::Id(id))
                    .await
                {
                    Ok((_, it)) => it,
                    Err(err) => RunResultBuild::default()
                        .set_status_msg(err.to_string())
                        .build(),
                }
            }
            else {
                RunResultBuild::default()
                    .set_status_msg("id lower 1".to_owned())
                    .build()
            };

            eve_tx
                .send(UserEvent::SubmitDone(Box::new(runres)))
                .expect("submit_code send failed");
        });
        false
    }

    pub fn test_code(&mut self) -> bool {
        let id = self
            .cur_qs
            .question_id
            .parse()
            .unwrap_or_default();

        // avoid repeated requests
        if self.edit.button.submitting {
            return false;
        }
        self.edit.button.start();

        let eve_tx = self.events.tx.clone();
        tokio::spawn(async move {
            // min id is 1
            let runres = if id > 0 {
                match glob_leetcode()
                    .await
                    .test_code(IdSlug::Id(id))
                    .await
                {
                    Ok((_, it)) => it,
                    Err(err) => RunResultBuild::default()
                        .set_status_msg(err.to_string())
                        .build(),
                }
            }
            else {
                RunResult::default()
            };
            eve_tx
                .send(UserEvent::TestDone(Box::new(runres)))
                .expect("test_code send failed");
        });
        false
    }
    pub fn test_done(&mut self, res: RunResult) {
        self.edit.test.content = res;

        self.edit.test.open();
        self.edit.submit.close();
        self.edit.button.close();

        self.edit.button.test_done();
        self.render();
    }
    pub fn submit_done(&mut self, res: RunResult) {
        self.edit.submit.need_add_test_case = !res.last_testcase.is_empty();
        self.edit.submit.content = res;

        self.edit.submit.open();
        self.edit.test.close();
        self.edit.button.close();

        self.edit.button.submit_done();
        self.render();
    }
}
