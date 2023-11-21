pub trait TpassOperation {
    fn extend_token();
    fn change_user();
    fn check_token();
    // fn naturemenu();
    fn tpass_request();
    // fn agreement_list_query();
    fn get_redirect();
    fn get_code_by_access_token();
    fn refresh_token();
}

pub trait TpassRequest {
    fn do_request();
}
