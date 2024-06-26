use std::{path::PathBuf, time::Duration};

use lcode_config::global::G_USER_CONFIG;
use leetcode_api::{
    glob_leetcode,
    leetcode::resps::{checkin::TotalPoints, pass_qs::PassData, user_data::UserStatus},
};
use notify_rust::Notification;
use tokio::join;

use crate::{app::inner::App, mytui::myevent::UserEvent};

impl<'app> App<'app> {
    /// get use info
    /// If haven't checked in, check in it.
    pub fn user_info_and_checkin(&self) {
        let tx = self.events.tx.clone();

        tokio::spawn(async move {
            let (user_status, points) = join!(
                glob_leetcode().await.get_user_info(),
                glob_leetcode().await.get_points()
            );

            if let Ok(mut status) = user_status {
                let avatar_path = glob_leetcode()
                    .await
                    .dow_user_avator(&status)
                    .await;

                let check = if !status.checked_in_today && status.user_slug.is_some() {
                    let (res_cn, res_com) = glob_leetcode()
                        .await
                        .daily_checkin()
                        .await;

                    let avatar_path = avatar_path
                        .as_os_str()
                        .to_str()
                        .unwrap_or_default();
                    if res_cn.checkin_ok() {
                        Notification::new()
                            .appname("lcode")
                            .summary("力扣签到")
                            .timeout(Duration::from_secs(2))
                            .body(&format!("{} 签到成功", status.username))
                            .icon(avatar_path)
                            .show()
                            .ok();
                    }

                    if res_com.checkin_ok() {
                        Notification::new()
                            .appname("lcode")
                            .summary("Leetcode Checkin")
                            .timeout(Duration::from_secs(2))
                            .body(&format!("{} checkin successful", status.username))
                            .icon(avatar_path)
                            .show()
                            .ok();
                    }
                    match G_USER_CONFIG.get_suffix() {
                        "cn" => res_cn.checkin_ok(),
                        _ => res_com.checkin_ok(),
                    }
                }
                else {
                    false
                };

                let ps_data = glob_leetcode()
                    .await
                    .pass_qs_status(
                        status
                            .user_slug
                            .as_deref()
                            .unwrap_or_default(),
                    )
                    .await
                    .unwrap_or_default();

                let mut points = points.unwrap_or_default();
                // update data
                if check {
                    status.checked_in_today = true;
                    points.add_point(1);
                }
                tx.send(UserEvent::UserInfo(Box::new((
                    status,
                    points,
                    ps_data,
                    avatar_path,
                ))))
                .ok();
            }
        });
    }

    pub fn get_status_done(&mut self, info: (UserStatus, TotalPoints, PassData, PathBuf)) {
        (
            self.info.user_status,
            self.info.points,
            self.info.pass_data,
            self.info.avatar_path,
        ) = info;

        self.render();
    }
}
