#![allow(unused, dead_code)]
use std::alloc::{ self, Layout };

pub fn raw_alloc<T>() -> *mut T {
	unsafe {
		alloc::alloc(size_oft::<T>()) as *mut T
	}
}

pub fn raw_dealloc<T>(ptr: *mut T) {
	unsafe {
		alloc::dealloc(ptr as *mut u8, size_oft::<T>())
	}
}

pub fn null_ptr<T>() -> *mut T {
	std::ptr::null_mut() as *mut T
}

fn size_ofv<T>(_: T) -> Layout {
	Layout::new::<T>()
}

fn size_oft<T>() -> Layout {
	Layout::new::<T>()
}