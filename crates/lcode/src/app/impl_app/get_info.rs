use std::time::Duration;

use leetcode_api::{
    glob_leetcode,
    leetcode::resps::{checkin::TotalPoints, user_data::UserStatus},
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

            if let Ok(status) = &user_status {
                let avatar_path = glob_leetcode()
                    .await
                    .dow_user_avator(status)
                    .await;
                let body = format!("{} checkin leetcode successful", status.username);

                if !status.checked_in_today && status.user_slug.is_some() {
                    let res = glob_leetcode()
                        .await
                        .daily_checkin()
                        .await;

                    if res.0.data.checkin.ok {
                        Notification::new()
                            .appname("lcode")
                            .summary("Leetcode.cn Checkin")
                            .timeout(Duration::from_secs(1))
                            .body(&body)
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
                            .summary("leetcode.com Checkin")
                            .timeout(Duration::from_secs(1))
                            .body(&body)
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
            tx.send(UserEvent::UserInfo(Box::new((
                user_status.unwrap_or_default(),
                points.unwrap_or_default(),
            ))))
        });
    }

    pub fn get_status_done(&mut self, info: (UserStatus, TotalPoints)) {
        (self.user_status, self.points) = info;
    }
}
