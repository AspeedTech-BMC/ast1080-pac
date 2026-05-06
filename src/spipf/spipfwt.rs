#[doc = "Register `SPIPFWT[%s]` reader"]
pub type R = crate::R<SpipfwtSpec>;
#[doc = "Register `SPIPFWT[%s]` writer"]
pub type W = crate::W<SpipfwtSpec>;
#[doc = "Field `Cmd` reader - Command"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `Cmd` writer - Command"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AddrBitWidth` reader - Address Bit width"]
pub type AddrBitWidthR = crate::FieldReader;
#[doc = "Field `AddrBitWidth` writer - Address Bit width"]
pub type AddrBitWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AddrMask` reader - Address Mask"]
pub type AddrMaskR = crate::FieldReader;
#[doc = "Field `AddrMask` writer - Address Mask"]
pub type AddrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ProgramOrEraseSize` reader - Program or Erase Size"]
pub type ProgramOrEraseSizeR = crate::FieldReader;
#[doc = "Field `ProgramOrEraseSize` writer - Program or Erase Size"]
pub type ProgramOrEraseSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NumberOfDummyCycles` reader - Number of Dummy Cycles"]
pub type NumberOfDummyCyclesR = crate::FieldReader;
#[doc = "Field `NumberOfDummyCycles` writer - Number of Dummy Cycles"]
pub type NumberOfDummyCyclesW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Lock` reader - Lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `Lock` writer - Lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DataBitWidth` reader - Data Bit width"]
pub type DataBitWidthR = crate::FieldReader;
#[doc = "Field `DataBitWidth` writer - Data Bit width"]
pub type DataBitWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MemoryCmd` reader - Memory Command"]
pub type MemoryCmdR = crate::BitReader;
#[doc = "Field `MemoryCmd` writer - Memory Command"]
pub type MemoryCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ReadCmd` reader - Read Command"]
pub type ReadCmdR = crate::BitReader;
#[doc = "Field `ReadCmd` writer - Read Command"]
pub type ReadCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrCmd` reader - Write Command"]
pub type WrCmdR = crate::BitReader;
#[doc = "Field `WrCmd` writer - Write Command"]
pub type WrCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Generic` reader - Generic"]
pub type GenericR = crate::BitReader;
#[doc = "Field `Generic` writer - Generic"]
pub type GenericW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Valid` reader - Valid"]
pub type ValidR = crate::BitReader;
#[doc = "Field `Valid` writer - Valid"]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Valid Once\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidOnce {
    #[doc = "0: 0]) comes."]
    _0Comes = 0,
}
impl From<ValidOnce> for bool {
    #[inline(always)]
    fn from(variant: ValidOnce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ValidOnce` reader - Valid Once"]
pub type ValidOnceR = crate::BitReader<ValidOnce>;
impl ValidOnceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ValidOnce> {
        match self.bits {
            false => Some(ValidOnce::_0Comes),
            _ => None,
        }
    }
    #[doc = "0]) comes."]
    #[inline(always)]
    pub fn is_0_comes(&self) -> bool {
        *self == ValidOnce::_0Comes
    }
}
#[doc = "Field `ValidOnce` writer - Valid Once"]
pub type ValidOnceW<'a, REG> = crate::BitWriter<'a, REG, ValidOnce>;
impl<'a, REG> ValidOnceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0]) comes."]
    #[inline(always)]
    pub fn _0_comes(self) -> &'a mut crate::W<REG> {
        self.variant(ValidOnce::_0Comes)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Address Bit width"]
    #[inline(always)]
    pub fn addr_bit_width(&self) -> AddrBitWidthR {
        AddrBitWidthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Address Mask"]
    #[inline(always)]
    pub fn addr_mask(&self) -> AddrMaskR {
        AddrMaskR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Program or Erase Size"]
    #[inline(always)]
    pub fn program_or_erase_size(&self) -> ProgramOrEraseSizeR {
        ProgramOrEraseSizeR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn number_of_dummy_cycles(&self) -> NumberOfDummyCyclesR {
        NumberOfDummyCyclesR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Data Bit width"]
    #[inline(always)]
    pub fn data_bit_width(&self) -> DataBitWidthR {
        DataBitWidthR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Memory Command"]
    #[inline(always)]
    pub fn memory_cmd(&self) -> MemoryCmdR {
        MemoryCmdR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Read Command"]
    #[inline(always)]
    pub fn read_cmd(&self) -> ReadCmdR {
        ReadCmdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Write Command"]
    #[inline(always)]
    pub fn wr_cmd(&self) -> WrCmdR {
        WrCmdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Generic"]
    #[inline(always)]
    pub fn generic(&self) -> GenericR {
        GenericR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Valid"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Valid Once"]
    #[inline(always)]
    pub fn valid_once(&self) -> ValidOnceR {
        ValidOnceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<SpipfwtSpec> {
        CmdW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Address Bit width"]
    #[inline(always)]
    pub fn addr_bit_width(&mut self) -> AddrBitWidthW<SpipfwtSpec> {
        AddrBitWidthW::new(self, 8)
    }
    #[doc = "Bits 10:12 - Address Mask"]
    #[inline(always)]
    pub fn addr_mask(&mut self) -> AddrMaskW<SpipfwtSpec> {
        AddrMaskW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Program or Erase Size"]
    #[inline(always)]
    pub fn program_or_erase_size(&mut self) -> ProgramOrEraseSizeW<SpipfwtSpec> {
        ProgramOrEraseSizeW::new(self, 13)
    }
    #[doc = "Bits 16:21 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn number_of_dummy_cycles(&mut self) -> NumberOfDummyCyclesW<SpipfwtSpec> {
        NumberOfDummyCyclesW::new(self, 16)
    }
    #[doc = "Bit 23 - Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<SpipfwtSpec> {
        LockW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Data Bit width"]
    #[inline(always)]
    pub fn data_bit_width(&mut self) -> DataBitWidthW<SpipfwtSpec> {
        DataBitWidthW::new(self, 24)
    }
    #[doc = "Bit 26 - Memory Command"]
    #[inline(always)]
    pub fn memory_cmd(&mut self) -> MemoryCmdW<SpipfwtSpec> {
        MemoryCmdW::new(self, 26)
    }
    #[doc = "Bit 27 - Read Command"]
    #[inline(always)]
    pub fn read_cmd(&mut self) -> ReadCmdW<SpipfwtSpec> {
        ReadCmdW::new(self, 27)
    }
    #[doc = "Bit 28 - Write Command"]
    #[inline(always)]
    pub fn wr_cmd(&mut self) -> WrCmdW<SpipfwtSpec> {
        WrCmdW::new(self, 28)
    }
    #[doc = "Bit 29 - Generic"]
    #[inline(always)]
    pub fn generic(&mut self) -> GenericW<SpipfwtSpec> {
        GenericW::new(self, 29)
    }
    #[doc = "Bit 30 - Valid"]
    #[inline(always)]
    pub fn valid(&mut self) -> ValidW<SpipfwtSpec> {
        ValidW::new(self, 30)
    }
    #[doc = "Bit 31 - Valid Once"]
    #[inline(always)]
    pub fn valid_once(&mut self) -> ValidOnceW<SpipfwtSpec> {
        ValidOnceW::new(self, 31)
    }
}
#[doc = "Allow Command Table \\#1 $\\sim$ \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`spipfwt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipfwt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpipfwtSpec;
impl crate::RegisterSpec for SpipfwtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipfwt::R`](R) reader structure"]
impl crate::Readable for SpipfwtSpec {}
#[doc = "`write(|w| ..)` method takes [`spipfwt::W`](W) writer structure"]
impl crate::Writable for SpipfwtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPFWT[%s] to value 0"]
impl crate::Resettable for SpipfwtSpec {}
