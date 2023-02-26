use alloc::vec;
use alloc::vec::Vec;

use uefi::proto::media::file::{Directory, File, FileInfo, FileMode, RegularFile};

use libs::error::LibResult;
use libs::kernel::entry_point::EntryPoint;
use libs::kernel::loaders::elf_loader::ElfLoader;
use libs::kernel::loaders::{Allocatable, KernelLoadable};

use crate::file::open_file;

pub fn load_kernel(
    root_dir: &mut Directory,
    kernel_file_path: &str,
    allocator: &mut impl Allocatable,
) -> LibResult<EntryPoint> {
    let mut kernel_file = open_file(root_dir, kernel_file_path, FileMode::Read)
        .map(|file_handle| file_handle.into_regular_file())
        .expect("should open kernel.libs")
        .unwrap();

    let kernel_file_size = get_kernel_file_size(&mut kernel_file) as usize;
    let tmp_buff_addr = allocator.allocate_pool(kernel_file_size);

    let mut kernel_vec = read_kernel_buff(&mut kernel_file, kernel_file_size);

    let result = ElfLoader::new().load(kernel_vec.as_mut_slice(), allocator);
    allocator.free_pool(tmp_buff_addr);
    kernel_file.close();
    result
}

fn get_kernel_file_size(kernel_file: &mut RegularFile) -> u64 {
    // カーネルファイルの大きさを知るため、ファイル情報を読み取る
    const FILE_INFO_SIZE: usize = 4000;

    let mut buff = Vec::<u8>::new();
    buff.resize(FILE_INFO_SIZE, 0);
    let info = kernel_file
        .get_info::<FileInfo>(buff.as_mut_slice())
        .expect("should obtain kernel libs info");

    info.file_size()
}

fn read_kernel_buff(kernel_file: &mut RegularFile, kernel_file_size: usize) -> Vec<u8> {
    let mut v = vec![0; kernel_file_size];
    kernel_file.read(v.as_mut_slice()).unwrap();
    v
}
