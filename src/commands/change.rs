// Copyright 2014-2017 The Rooster Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clip;
use ffi;
use list;
use macros::show_error;
use password;
use rpassword::prompt_password_stderr;
use safe_string::SafeString;

pub fn callback_exec(
    matches: &clap::ArgMatches,
    store: &mut password::v2::PasswordStore,
) -> Result<(), i32> {
    let query = matches.value_of("app").unwrap();

    let password = list::search_and_choose_password(
        store,
        query,
        list::WITH_NUMBERS,
        "Which password would like to update?",
    )
    .ok_or(1)?
    .clone();

    let password_as_string = prompt_password_stderr(
        format!("What password do you want for \"{}\"? ", password.name).as_str(),
    )
    .map_err(|err| {
        show_error(format!("\nI couldn't read the app's password (reason: {:?}).", err).as_str());
        1
    })?;

    let password_as_string = SafeString::new(password_as_string);

    let password = store
        .change_password(&password.name, &|old_password: password::v2::Password| {
            password::v2::Password {
                name: old_password.name,
                username: old_password.username,
                password: password_as_string.clone(),
                created_at: old_password.created_at,
                updated_at: ffi::time(),
            }
        })
        .map_err(|err| {
            show_error(
                format!(
                    "Woops, I couldn't save the new password (reason: {:?}).",
                    err
                )
                .as_str(),
            );
            1
        })?;

    let show = matches.is_present("show");
    clip::confirm_password_retrieved(show, &password);
    Ok(())
}
