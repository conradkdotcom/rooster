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

use std::ops::Drop;
use std::ops::Deref;
use std::ops::DerefMut;
use std::{ptr, sync::atomic};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SafeVec {
    pub inner: Vec<u8>,
}

impl SafeVec {
    pub fn new(inner: Vec<u8>) -> SafeVec {
        SafeVec { inner: inner }
    }

    pub fn inner_mut(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}

impl Drop for SafeVec {
    fn drop(&mut self) {
        let default = u8::default();

        for c in self.inner.as_mut_slice() {
            unsafe { ptr::write_volatile(c, default) };
        }

        atomic::fence(atomic::Ordering::SeqCst);
        atomic::compiler_fence(atomic::Ordering::SeqCst);
    }
}

impl Deref for SafeVec {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.inner.deref()
    }
}

impl DerefMut for SafeVec {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.inner.deref_mut()
    }
}
