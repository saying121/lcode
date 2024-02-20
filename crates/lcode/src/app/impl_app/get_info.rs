use std::time::Duration;

use leetcode_api::{
    glob_leetcode,
    leetcode::resps::{checkin::TotalPoints, pass_qs::PassData, user_data::UserStatus},
};
use notify_rust::Notification;
use tokio::join;

use crate::{app::inner::App, mytui::myevent::UserEvent};

impl<'app> App<'app> {
    pub fn user_info_and_checkin(&self) {
        let tx = self.events.tx.clone();

        tokio::spawn(async move {
            let (user_status, points) = join!(
                glob_leetcode().await.get_user_info(),
                glob_leetcode().await.get_points()
            );
            let ps_data;

            if let Ok(status) = &user_status {
                ps_data = glob_leetcode()
                    .await
                    .pass_qs_status(
                        status
                            .user_slug
                            .as_deref()
                            .unwrap_or_default(),
                    )
                    .await
                    .unwrap_or_default();

                let avatar_path = glob_leetcode()
                    .await
                    .dow_user_avator(status)
                    .await;

                if !status.checked_in_today && status.user_slug.is_some() {
                    let res = glob_leetcode()
                        .await
                        .daily_checkin()
                        .await;

                    if res.0.data.checkin.ok {
                        Notification::new()
                            .appname("lcode")
                            .summary("力扣签到")
                            .timeout(Duration::from_secs(2))
                            .body(&format!("{} 签到成功", status.username))
                            .icon(
                                avatar_path
                                    .as_os_str()
                                    .to_str()
                                    .unwrap_or_default(),
                            )
                            .show()
                            .ok();
                    }
                    if res.1.data.checkin.ok {
                        Notification::new()
                            .appname("lcode")
                            .summary("Leetcode Checkin")
                            .timeout(Duration::from_secs(2))
                            .body(&format!("{} checkin successful", status.username))
                            .icon(
                                avatar_path
                                    .as_os_str()
                                    .to_str()
                                    .unwrap_or_default(),
                            )
                            .show()
                            .ok();
                    }
                }
            }
            else {
                ps_data = PassData::default();
            }

            tx.send(UserEvent::UserInfo(Box::new((
                user_status.unwrap_or_default(),
                points.unwrap_or_default(),
                ps_data,
            ))))
        });
    }

    pub fn get_status_done(&mut self, info: (UserStatus, TotalPoints, PassData)) {
        (
            self.infos.user_status,
            self.infos.points,
            self.infos.pass_data,
        ) = info;
    }
}
