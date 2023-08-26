use std::sync::OnceLock;

pub(super) static PROBLEMSETQUESTIONLIST: OnceLock<Vec<&str>> = OnceLock::new();
// ","variables":{"skip":0,"limit":4000,"filters":{}},"operationName":"problemsetQuestionList"}' \
pub(super) fn init_pbsetlist_grql() -> Vec<&'static str> {
    PROBLEMSETQUESTIONLIST
        .get_or_init(|| {
            vec![
            "query problemsetQuestionList(",
            "    $limit: Int",
            "    $skip: Int",
            "    $filters: QuestionListFilterInput",
            ") {",
            "    problemsetQuestionList(limit: $limit, skip: $skip, filters: $filters) {",
            "        hasMore",
            "        total",
            "        questions {",
            "            acRate",
            "            difficulty",
            "            freqBar",
            "            frontendQuestionId",
            "            isFavor",
            "            paidOnly",
            "            solutionNum",
            "            status",
            "            title",
            "            titleCn",
            "            titleSlug",
            "            topicTags {",
            "                name",
            "                nameTranslated",
            "                id",
            "                slug",
            "            }",
            "        }",
            "    }",
            "}",
        ]
        })
        .to_vec()
}

pub(super) static QS_DETAIL_GRAPHQL: OnceLock<Vec<&str>> = OnceLock::new();
pub(super) fn init_qs_dt_grql() -> Vec<&'static str> {
    QS_DETAIL_GRAPHQL
        .get_or_init(|| {
            vec![
                "query getQuestion($titleSlug: String!) {",
                "    question(titleSlug: $titleSlug) {",
                "        content",          // question content
                "        stats", // question pass submit status 题目通过/提交.etc状态
                "        sampleTestCase", // test case
                "        exampleTestcases", // example
                "        metaData",
                "        translatedTitle",   // translate
                "        translatedContent", // translate
                "        hints",
                "        mysqlSchemas",
                "        dataSchemas",
                "        questionId",
                "        questionTitle",
                "        isPaidOnly",
                "        codeSnippets {",
                "            lang",
                "            langSlug",
                "            code", // question template
                "        }",
                "        title",
                "        isPaidOnly",
                "        difficulty",
                "        topicTags {",
                "            name", // category name
                "            slug",
                "            translatedName", // translate
                "        }",
                "    }",
                "}",
            ]
        })
        .to_vec()
}

pub(super) static QUERY_SUBMISSION_LIST_GRAPHQL: OnceLock<Vec<&str>> = OnceLock::new();
pub(super) fn init_subit_list_grql() -> Vec<&'static str> {
    QUERY_SUBMISSION_LIST_GRAPHQL
        .get_or_init(|| {
            vec![
                "query submissionList(",
                "    $offset: Int!",
                "    $limit: Int!",
                "    $lastKey: String",
                "    $questionSlug: String!",
                "    $lang: String",
                "    $status: SubmissionStatusEnum",
                ") {",
                "    submissionList(",
                "        offset: $offset",
                "        limit: $limit",
                "        lastKey: $lastKey",
                "        questionSlug: $questionSlug",
                "        lang: $lang",
                "        status: $status",
                "    ) {",
                "        lastKey",
                "        hasNext",
                "        submissions {",
                "            id",
                "            title",
                "            status",
                "            statusDisplay",
                "            lang",
                "            langName: langVerboseName",
                "            runtime",
                "            timestamp",
                "            url",
                "            isPending",
                "            memory",
                "            submissionComment {",
                "                comment",
                "            } ",
                "        } ",
                "    } ",
                "}",
            ]
        })
        .to_vec()
}
