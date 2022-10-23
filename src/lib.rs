#![allow(non_snake_case)]
use binrw::*;

#[binrw]
#[derive(Clone)]
pub struct NRO {
    pub start: NROStart,
    pub header: NROHeader,
    #[br(count(header.size - 0x80))]
    pub data: Vec<u8>,
    #[br(try)]
    pub assets: Option<Assets>
}

#[binrw]
#[derive(Clone, Copy)]
pub struct NROStart {
    pub unused: u32,
    pub MOD0Offset: u32,
    pub padding: [u8; 0x8],
}

#[binrw]
#[br(magic=b"NRO0")]
#[derive(Clone, Copy)]
pub struct NROHeader {
    pub version: u32, // Always 0
    pub size: u32, // Full NRO file size
    pub flags: u32, // unused
    pub textSegmentHeader: SegmentHeader,
    pub roSegmentHeader: SegmentHeader,
    pub dataSegmentHeader: SegmentHeader,
    pub BssSize: u32,
    pub reserved: [u8; 0x4],
    pub ModuleID: [u8; 0x20],
    pub DsoHandleOffset: u32, // unused
    pub reserved_2: [u8; 0x4],
    pub apiInfoSegmentHeader: SegmentHeader,
    pub dynstrSegmentHeader: SegmentHeader,
    pub dynsymSegmentHeader: SegmentHeader
}

#[binrw]
#[derive(Clone, Copy)]
pub struct SegmentHeader {
    pub MemoryOffset: u32,
    pub size: u32
}

#[binrw]
#[derive(Clone)]
pub struct Assets { // Goes unused in cargo-skyline generated NROs
    pub assetHeader: AssetHeader,

    #[br(offset = assetHeader.iconAssetSection.offset)]
    #[br(if(assetHeader.iconAssetSection.size != 0))]
    #[br(count(assetHeader.iconAssetSection.size))]
    pub icon: Option<Vec<u8>>,

    #[br(offset = assetHeader.nacpAssetSection.offset)]
    #[br(if(assetHeader.nacpAssetSection.size != 0))]
    #[br(count(assetHeader.nacpAssetSection.size))]
    pub nacp: Option<Vec<u8>>,

    #[br(offset = assetHeader.romfsAssetSection.offset)]
    #[br(if(assetHeader.romfsAssetSection.size != 0))]
    #[br(count(assetHeader.romfsAssetSection.size))]
    pub romfs: Option<Vec<u8>>
}

#[binrw]
#[br(magic=b"ASET")]
#[derive(Clone, Copy)]
pub struct AssetHeader {
    pub version: u32,
    pub iconAssetSection: AssetSection,
    pub nacpAssetSection: AssetSection,
    pub romfsAssetSection: AssetSection
}

#[binrw]
#[derive(Clone, Copy)]
pub struct AssetSection {
    pub offset: u64,
    pub size: u64
}
