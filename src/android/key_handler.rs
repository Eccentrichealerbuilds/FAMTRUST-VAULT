use serde::{Serialize, de::DeserializeOwned};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

const FILE_NAME: &str = "keys.json";

#[cfg(target_os = "android")]
fn get_files_dir() -> Result<PathBuf, String> {
    let jni_result =
        |env: &mut jni::JNIEnv, activity: &jni::objects::JObject| -> Result<PathBuf, String> {
            let files_dir = env
                .call_method(&activity, "getFilesDir", "()Ljava/io/File;", &[])
                .and_then(|value| value.l())
                .map_err(|error| error.to_string())?;

            let path_jstring: jni::objects::JString = env
                .call_method(&files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])
                .and_then(|value| value.l())
                .map_err(|error| error.to_string())?
                .into();

            let path: String = env
                .get_string(&path_jstring)
                .map_err(|error| error.to_string())?
                .into();

            Ok(PathBuf::from(path))
        };

    let activity_result =
        dioxus::prelude::with_activity(|mut env, activity| Some(jni_result(&mut env, &activity)));

    if let None = activity_result {
        return Err(String::from("Unable to get enviroment"));
    }
    Ok(activity_result.unwrap()?)
}

#[cfg(target_os = "android")]
fn key_path() -> Result<PathBuf, String> {
    Ok(get_files_dir()?.join(FILE_NAME))
}

#[cfg(target_os = "android")]
pub fn save_json<T: Serialize>(keymap: &BTreeMap<String, T>) -> Result<(), String> {
    let path = key_path()?;
    let contents = serde_json::to_string(keymap).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
}

#[cfg(target_os = "android")]
pub fn load_json<T: DeserializeOwned>() -> Result<BTreeMap<String, T>, String> {
    let path = key_path()?;
    if !path.exists() {
        return Ok(BTreeMap::new());
    }

    let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&contents).map_err(|error| error.to_string())
}
