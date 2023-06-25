use tts_external_api::{messages::AnswerReload, ExternalEditorApi};

fn main() {
    let api = ExternalEditorApi::new();
    get_lua_scripts(api)
}

fn get_lua_scripts(api: ExternalEditorApi) {
    let answer_reload: AnswerReload = api.get_scripts().unwrap();
    println!("{:#?}", answer_reload.script_states);
}
