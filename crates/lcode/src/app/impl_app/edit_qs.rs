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
                .unwrap_or_default();
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
    pub fn submit_code(&mut self) -> bool {
        let id: u32 = self
            .cur_qs
            .question_id
            .parse()
            .unwrap_or_default();

        // avoid repeated requests
        if self.edit.submitting {
            return false;
        }

        self.edit.submitting = true;
        let eve_tx = self.events.tx.clone();
        tokio::spawn(async move {
            // min id is 1
            let (_, temp) = if id > 0 {
                glob_leetcode()
                    .await
                    .submit_code(IdSlug::Id(id))
                    .await
                    .unwrap_or_default()
            }
            else {
                (SubmitInfo::default(), RunResult::default())
            };

            // update infos
            if temp.total_correct == temp.total_testcases {
                self.user_info_and_checkin();
            }
            eve_tx
                .send(UserEvent::SubmitDone(Box::new(temp)))
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
        if self.edit.submitting {
            return false;
        }
        self.edit.submitting = true;

        let eve_tx = self.events.tx.clone();
        tokio::spawn(async move {
            // min id is 1
            let temp = if id > 0 {
                glob_leetcode()
                    .await
                    .test_code(IdSlug::Id(id))
                    .await
                    .unwrap_or_default()
            }
            else {
                (TestInfo::default(), RunResult::default())
            };
            eve_tx
                .send(UserEvent::TestDone(Box::new(temp.1)))
                .expect("test_code send failed");
        });
        false
    }
    pub fn submit_done(&mut self, res: RunResult) {
        self.edit.submit_res = res;
        self.edit.show_submit_res = true;
        self.edit.submitting = false;
        self.render();
    }
    pub fn test_done(&mut self, res: RunResult) {
        self.edit.test_res = res;
        self.edit.show_test_res = true;
        self.edit.submitting = false;
        self.render();
    }
}
