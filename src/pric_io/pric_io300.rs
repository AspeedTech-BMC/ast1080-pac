#[doc = "Register `PRIC_IO300` reader"]
pub type R = crate::R<PricIo300Spec>;
#[doc = "Register `PRIC_IO300` writer"]
pub type W = crate::W<PricIo300Spec>;
#[doc = "Field `EnblReadGroup0OfH2M` reader - Enable Read Group #0 of H2M"]
pub type EnblReadGroup0ofH2mR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfH2M` writer - Enable Read Group #0 of H2M"]
pub type EnblReadGroup0ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfH2M` reader - Enable Read Group #1 of H2M"]
pub type EnblReadGroup1ofH2mR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfH2M` writer - Enable Read Group #1 of H2M"]
pub type EnblReadGroup1ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfH2M` reader - Enable Read Group #2 of H2M"]
pub type EnblReadGroup2ofH2mR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfH2M` writer - Enable Read Group #2 of H2M"]
pub type EnblReadGroup2ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfH2M` reader - Enable Read Group #3 of H2M"]
pub type EnblReadGroup3ofH2mR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfH2M` writer - Enable Read Group #3 of H2M"]
pub type EnblReadGroup3ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfH2M` reader - Enable Read Group #4 of H2M"]
pub type EnblReadGroup4ofH2mR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfH2M` writer - Enable Read Group #4 of H2M"]
pub type EnblReadGroup4ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfH2M` reader - Enable Read Group #5 of H2M"]
pub type EnblReadGroup5ofH2mR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfH2M` writer - Enable Read Group #5 of H2M"]
pub type EnblReadGroup5ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1300PRIC1_300\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1300pric13000500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1300pric13000500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1300pric13000500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1300PRIC13000500` reader - Enable Reset Tolerance of PRIC1300PRIC1_300\\[05:00\\]"]
pub type EnblRstToleranceOfPric1300pric13000500R =
    crate::BitReader<EnblRstToleranceOfPric1300pric13000500>;
impl EnblRstToleranceOfPric1300pric13000500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1300pric13000500 {
        match self.bits {
            false => EnblRstToleranceOfPric1300pric13000500::ResetBySrst,
            true => EnblRstToleranceOfPric1300pric13000500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1300pric13000500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1300pric13000500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1300PRIC13000500` writer - Enable Reset Tolerance of PRIC1300PRIC1_300\\[05:00\\]"]
pub type EnblRstToleranceOfPric1300pric13000500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1300pric13000500>;
impl<'a, REG> EnblRstToleranceOfPric1300pric13000500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1300pric13000500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1300pric13000500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1300PRIC13000600` reader - Enable Write Protection of PRIC1300PRIC1_300\\[06:00\\]"]
pub type EnblWrProtOfPric1300pric13000600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1300PRIC13000600` writer - Enable Write Protection of PRIC1300PRIC1_300\\[06:00\\]"]
pub type EnblWrProtOfPric1300pric13000600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfFMCMemory` reader - Enable Read Group #0 of FMC Memory"]
pub type EnblReadGroup0ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfFMCMemory` writer - Enable Read Group #0 of FMC Memory"]
pub type EnblReadGroup0ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfFMCMemory` reader - Enable Read Group #1 of FMC Memory"]
pub type EnblReadGroup1ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfFMCMemory` writer - Enable Read Group #1 of FMC Memory"]
pub type EnblReadGroup1ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfFMCMemory` reader - Enable Read Group #2 of FMC Memory"]
pub type EnblReadGroup2ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfFMCMemory` writer - Enable Read Group #2 of FMC Memory"]
pub type EnblReadGroup2ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfFMCMemory` reader - Enable Read Group #3 of FMC Memory"]
pub type EnblReadGroup3ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfFMCMemory` writer - Enable Read Group #3 of FMC Memory"]
pub type EnblReadGroup3ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfFMCMemory` reader - Enable Read Group #4 of FMC Memory"]
pub type EnblReadGroup4ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfFMCMemory` writer - Enable Read Group #4 of FMC Memory"]
pub type EnblReadGroup4ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfFMCMemory` reader - Enable Read Group #5 of FMC Memory"]
pub type EnblReadGroup5ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfFMCMemory` writer - Enable Read Group #5 of FMC Memory"]
pub type EnblReadGroup5ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1300PRIC1_300\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1300pric13001308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1300pric13001308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1300pric13001308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1300PRIC13001308` reader - Enable Reset Tolerance of PRIC1300PRIC1_300\\[13:08\\]"]
pub type EnblRstToleranceOfPric1300pric13001308R =
    crate::BitReader<EnblRstToleranceOfPric1300pric13001308>;
impl EnblRstToleranceOfPric1300pric13001308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1300pric13001308 {
        match self.bits {
            false => EnblRstToleranceOfPric1300pric13001308::ResetBySrst,
            true => EnblRstToleranceOfPric1300pric13001308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1300pric13001308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1300pric13001308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1300PRIC13001308` writer - Enable Reset Tolerance of PRIC1300PRIC1_300\\[13:08\\]"]
pub type EnblRstToleranceOfPric1300pric13001308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1300pric13001308>;
impl<'a, REG> EnblRstToleranceOfPric1300pric13001308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1300pric13001308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1300pric13001308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1300PRIC13001408` reader - Enable Write Protection of PRIC1300PRIC1_300\\[14:08\\]"]
pub type EnblWrProtOfPric1300pric13001408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1300PRIC13001408` writer - Enable Write Protection of PRIC1300PRIC1_300\\[14:08\\]"]
pub type EnblWrProtOfPric1300pric13001408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSPI0Memory` reader - Enable Read Group #0 of SPI0 Memory"]
pub type EnblReadGroup0ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSPI0Memory` writer - Enable Read Group #0 of SPI0 Memory"]
pub type EnblReadGroup0ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSPI0Memory` reader - Enable Read Group #1 of SPI0 Memory"]
pub type EnblReadGroup1ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSPI0Memory` writer - Enable Read Group #1 of SPI0 Memory"]
pub type EnblReadGroup1ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSPI0Memory` reader - Enable Read Group #2 of SPI0 Memory"]
pub type EnblReadGroup2ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSPI0Memory` writer - Enable Read Group #2 of SPI0 Memory"]
pub type EnblReadGroup2ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSPI0Memory` reader - Enable Read Group #3 of SPI0 Memory"]
pub type EnblReadGroup3ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSPI0Memory` writer - Enable Read Group #3 of SPI0 Memory"]
pub type EnblReadGroup3ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSPI0Memory` reader - Enable Read Group #4 of SPI0 Memory"]
pub type EnblReadGroup4ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSPI0Memory` writer - Enable Read Group #4 of SPI0 Memory"]
pub type EnblReadGroup4ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSPI0Memory` reader - Enable Read Group #5 of SPI0 Memory"]
pub type EnblReadGroup5ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSPI0Memory` writer - Enable Read Group #5 of SPI0 Memory"]
pub type EnblReadGroup5ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1300PRIC1_300\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1300pric13002116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1300pric13002116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1300pric13002116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1300PRIC13002116` reader - Enable Reset Tolerance of PRIC1300PRIC1_300\\[21:16\\]"]
pub type EnblRstToleranceOfPric1300pric13002116R =
    crate::BitReader<EnblRstToleranceOfPric1300pric13002116>;
impl EnblRstToleranceOfPric1300pric13002116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1300pric13002116 {
        match self.bits {
            false => EnblRstToleranceOfPric1300pric13002116::ResetBySrst,
            true => EnblRstToleranceOfPric1300pric13002116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1300pric13002116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1300pric13002116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1300PRIC13002116` writer - Enable Reset Tolerance of PRIC1300PRIC1_300\\[21:16\\]"]
pub type EnblRstToleranceOfPric1300pric13002116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1300pric13002116>;
impl<'a, REG> EnblRstToleranceOfPric1300pric13002116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1300pric13002116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1300pric13002116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1300PRIC13002216` reader - Enable Write Protection of PRIC1300PRIC1_300\\[22:16\\]"]
pub type EnblWrProtOfPric1300pric13002216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1300PRIC13002216` writer - Enable Write Protection of PRIC1300PRIC1_300\\[22:16\\]"]
pub type EnblWrProtOfPric1300pric13002216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSPI1Memory` reader - Enable Read Group #0 of SPI1 Memory"]
pub type EnblReadGroup0ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSPI1Memory` writer - Enable Read Group #0 of SPI1 Memory"]
pub type EnblReadGroup0ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSPI1Memory` reader - Enable Read Group #1 of SPI1 Memory"]
pub type EnblReadGroup1ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSPI1Memory` writer - Enable Read Group #1 of SPI1 Memory"]
pub type EnblReadGroup1ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSPI1Memory` reader - Enable Read Group #2 of SPI1 Memory"]
pub type EnblReadGroup2ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSPI1Memory` writer - Enable Read Group #2 of SPI1 Memory"]
pub type EnblReadGroup2ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSPI1Memory` reader - Enable Read Group #3 of SPI1 Memory"]
pub type EnblReadGroup3ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSPI1Memory` writer - Enable Read Group #3 of SPI1 Memory"]
pub type EnblReadGroup3ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSPI1Memory` reader - Enable Read Group #4 of SPI1 Memory"]
pub type EnblReadGroup4ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSPI1Memory` writer - Enable Read Group #4 of SPI1 Memory"]
pub type EnblReadGroup4ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSPI1Memory` reader - Enable Read Group #5 of SPI1 Memory"]
pub type EnblReadGroup5ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSPI1Memory` writer - Enable Read Group #5 of SPI1 Memory"]
pub type EnblReadGroup5ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1300PRIC1_300\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1300pric13002924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1300pric13002924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1300pric13002924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1300PRIC13002924` reader - Enable Reset Tolerance of PRIC1300PRIC1_300\\[29:24\\]"]
pub type EnblRstToleranceOfPric1300pric13002924R =
    crate::BitReader<EnblRstToleranceOfPric1300pric13002924>;
impl EnblRstToleranceOfPric1300pric13002924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1300pric13002924 {
        match self.bits {
            false => EnblRstToleranceOfPric1300pric13002924::ResetBySrst,
            true => EnblRstToleranceOfPric1300pric13002924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1300pric13002924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1300pric13002924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1300PRIC13002924` writer - Enable Reset Tolerance of PRIC1300PRIC1_300\\[29:24\\]"]
pub type EnblRstToleranceOfPric1300pric13002924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1300pric13002924>;
impl<'a, REG> EnblRstToleranceOfPric1300pric13002924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1300pric13002924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1300pric13002924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1300PRIC13003024` reader - Enable Write Protection of PRIC1300PRIC1_300\\[30:24\\]"]
pub type EnblWrProtOfPric1300pric13003024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1300PRIC13003024` writer - Enable Write Protection of PRIC1300PRIC1_300\\[30:24\\]"]
pub type EnblWrProtOfPric1300pric13003024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group0of_h2m(&self) -> EnblReadGroup0ofH2mR {
        EnblReadGroup0ofH2mR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group1of_h2m(&self) -> EnblReadGroup1ofH2mR {
        EnblReadGroup1ofH2mR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group2of_h2m(&self) -> EnblReadGroup2ofH2mR {
        EnblReadGroup2ofH2mR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group3of_h2m(&self) -> EnblReadGroup3ofH2mR {
        EnblReadGroup3ofH2mR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group4of_h2m(&self) -> EnblReadGroup4ofH2mR {
        EnblReadGroup4ofH2mR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group5of_h2m(&self) -> EnblReadGroup5ofH2mR {
        EnblReadGroup5ofH2mR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1300PRIC1_300\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1300pric13000500(
        &self,
    ) -> EnblRstToleranceOfPric1300pric13000500R {
        EnblRstToleranceOfPric1300pric13000500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1300PRIC1_300\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1300pric13000600(&self) -> EnblWrProtOfPric1300pric13000600R {
        EnblWrProtOfPric1300pric13000600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group0of_fmcmemory(&self) -> EnblReadGroup0ofFmcmemoryR {
        EnblReadGroup0ofFmcmemoryR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group1of_fmcmemory(&self) -> EnblReadGroup1ofFmcmemoryR {
        EnblReadGroup1ofFmcmemoryR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group2of_fmcmemory(&self) -> EnblReadGroup2ofFmcmemoryR {
        EnblReadGroup2ofFmcmemoryR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group3of_fmcmemory(&self) -> EnblReadGroup3ofFmcmemoryR {
        EnblReadGroup3ofFmcmemoryR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group4of_fmcmemory(&self) -> EnblReadGroup4ofFmcmemoryR {
        EnblReadGroup4ofFmcmemoryR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group5of_fmcmemory(&self) -> EnblReadGroup5ofFmcmemoryR {
        EnblReadGroup5ofFmcmemoryR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1300PRIC1_300\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1300pric13001308(
        &self,
    ) -> EnblRstToleranceOfPric1300pric13001308R {
        EnblRstToleranceOfPric1300pric13001308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1300PRIC1_300\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1300pric13001408(&self) -> EnblWrProtOfPric1300pric13001408R {
        EnblWrProtOfPric1300pric13001408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi0memory(&self) -> EnblReadGroup0ofSpi0memoryR {
        EnblReadGroup0ofSpi0memoryR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi0memory(&self) -> EnblReadGroup1ofSpi0memoryR {
        EnblReadGroup1ofSpi0memoryR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi0memory(&self) -> EnblReadGroup2ofSpi0memoryR {
        EnblReadGroup2ofSpi0memoryR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi0memory(&self) -> EnblReadGroup3ofSpi0memoryR {
        EnblReadGroup3ofSpi0memoryR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi0memory(&self) -> EnblReadGroup4ofSpi0memoryR {
        EnblReadGroup4ofSpi0memoryR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi0memory(&self) -> EnblReadGroup5ofSpi0memoryR {
        EnblReadGroup5ofSpi0memoryR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1300PRIC1_300\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1300pric13002116(
        &self,
    ) -> EnblRstToleranceOfPric1300pric13002116R {
        EnblRstToleranceOfPric1300pric13002116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1300PRIC1_300\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1300pric13002216(&self) -> EnblWrProtOfPric1300pric13002216R {
        EnblWrProtOfPric1300pric13002216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi1memory(&self) -> EnblReadGroup0ofSpi1memoryR {
        EnblReadGroup0ofSpi1memoryR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi1memory(&self) -> EnblReadGroup1ofSpi1memoryR {
        EnblReadGroup1ofSpi1memoryR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi1memory(&self) -> EnblReadGroup2ofSpi1memoryR {
        EnblReadGroup2ofSpi1memoryR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi1memory(&self) -> EnblReadGroup3ofSpi1memoryR {
        EnblReadGroup3ofSpi1memoryR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi1memory(&self) -> EnblReadGroup4ofSpi1memoryR {
        EnblReadGroup4ofSpi1memoryR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi1memory(&self) -> EnblReadGroup5ofSpi1memoryR {
        EnblReadGroup5ofSpi1memoryR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1300PRIC1_300\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1300pric13002924(
        &self,
    ) -> EnblRstToleranceOfPric1300pric13002924R {
        EnblRstToleranceOfPric1300pric13002924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1300PRIC1_300\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1300pric13003024(&self) -> EnblWrProtOfPric1300pric13003024R {
        EnblWrProtOfPric1300pric13003024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group0of_h2m(&mut self) -> EnblReadGroup0ofH2mW<PricIo300Spec> {
        EnblReadGroup0ofH2mW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group1of_h2m(&mut self) -> EnblReadGroup1ofH2mW<PricIo300Spec> {
        EnblReadGroup1ofH2mW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group2of_h2m(&mut self) -> EnblReadGroup2ofH2mW<PricIo300Spec> {
        EnblReadGroup2ofH2mW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group3of_h2m(&mut self) -> EnblReadGroup3ofH2mW<PricIo300Spec> {
        EnblReadGroup3ofH2mW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group4of_h2m(&mut self) -> EnblReadGroup4ofH2mW<PricIo300Spec> {
        EnblReadGroup4ofH2mW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of H2M"]
    #[inline(always)]
    pub fn enbl_read_group5of_h2m(&mut self) -> EnblReadGroup5ofH2mW<PricIo300Spec> {
        EnblReadGroup5ofH2mW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1300PRIC1_300\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1300pric13000500(
        &mut self,
    ) -> EnblRstToleranceOfPric1300pric13000500W<PricIo300Spec> {
        EnblRstToleranceOfPric1300pric13000500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1300PRIC1_300\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1300pric13000600(
        &mut self,
    ) -> EnblWrProtOfPric1300pric13000600W<PricIo300Spec> {
        EnblWrProtOfPric1300pric13000600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group0of_fmcmemory(&mut self) -> EnblReadGroup0ofFmcmemoryW<PricIo300Spec> {
        EnblReadGroup0ofFmcmemoryW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group1of_fmcmemory(&mut self) -> EnblReadGroup1ofFmcmemoryW<PricIo300Spec> {
        EnblReadGroup1ofFmcmemoryW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group2of_fmcmemory(&mut self) -> EnblReadGroup2ofFmcmemoryW<PricIo300Spec> {
        EnblReadGroup2ofFmcmemoryW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group3of_fmcmemory(&mut self) -> EnblReadGroup3ofFmcmemoryW<PricIo300Spec> {
        EnblReadGroup3ofFmcmemoryW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group4of_fmcmemory(&mut self) -> EnblReadGroup4ofFmcmemoryW<PricIo300Spec> {
        EnblReadGroup4ofFmcmemoryW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_read_group5of_fmcmemory(&mut self) -> EnblReadGroup5ofFmcmemoryW<PricIo300Spec> {
        EnblReadGroup5ofFmcmemoryW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1300PRIC1_300\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1300pric13001308(
        &mut self,
    ) -> EnblRstToleranceOfPric1300pric13001308W<PricIo300Spec> {
        EnblRstToleranceOfPric1300pric13001308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1300PRIC1_300\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1300pric13001408(
        &mut self,
    ) -> EnblWrProtOfPric1300pric13001408W<PricIo300Spec> {
        EnblWrProtOfPric1300pric13001408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi0memory(&mut self) -> EnblReadGroup0ofSpi0memoryW<PricIo300Spec> {
        EnblReadGroup0ofSpi0memoryW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi0memory(&mut self) -> EnblReadGroup1ofSpi0memoryW<PricIo300Spec> {
        EnblReadGroup1ofSpi0memoryW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi0memory(&mut self) -> EnblReadGroup2ofSpi0memoryW<PricIo300Spec> {
        EnblReadGroup2ofSpi0memoryW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi0memory(&mut self) -> EnblReadGroup3ofSpi0memoryW<PricIo300Spec> {
        EnblReadGroup3ofSpi0memoryW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi0memory(&mut self) -> EnblReadGroup4ofSpi0memoryW<PricIo300Spec> {
        EnblReadGroup4ofSpi0memoryW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi0memory(&mut self) -> EnblReadGroup5ofSpi0memoryW<PricIo300Spec> {
        EnblReadGroup5ofSpi0memoryW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1300PRIC1_300\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1300pric13002116(
        &mut self,
    ) -> EnblRstToleranceOfPric1300pric13002116W<PricIo300Spec> {
        EnblRstToleranceOfPric1300pric13002116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1300PRIC1_300\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1300pric13002216(
        &mut self,
    ) -> EnblWrProtOfPric1300pric13002216W<PricIo300Spec> {
        EnblWrProtOfPric1300pric13002216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi1memory(&mut self) -> EnblReadGroup0ofSpi1memoryW<PricIo300Spec> {
        EnblReadGroup0ofSpi1memoryW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi1memory(&mut self) -> EnblReadGroup1ofSpi1memoryW<PricIo300Spec> {
        EnblReadGroup1ofSpi1memoryW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi1memory(&mut self) -> EnblReadGroup2ofSpi1memoryW<PricIo300Spec> {
        EnblReadGroup2ofSpi1memoryW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi1memory(&mut self) -> EnblReadGroup3ofSpi1memoryW<PricIo300Spec> {
        EnblReadGroup3ofSpi1memoryW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi1memory(&mut self) -> EnblReadGroup4ofSpi1memoryW<PricIo300Spec> {
        EnblReadGroup4ofSpi1memoryW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi1memory(&mut self) -> EnblReadGroup5ofSpi1memoryW<PricIo300Spec> {
        EnblReadGroup5ofSpi1memoryW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1300PRIC1_300\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1300pric13002924(
        &mut self,
    ) -> EnblRstToleranceOfPric1300pric13002924W<PricIo300Spec> {
        EnblRstToleranceOfPric1300pric13002924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1300PRIC1_300\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1300pric13003024(
        &mut self,
    ) -> EnblWrProtOfPric1300pric13003024W<PricIo300Spec> {
        EnblWrProtOfPric1300pric13003024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io300::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io300::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo300Spec;
impl crate::RegisterSpec for PricIo300Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io300::R`](R) reader structure"]
impl crate::Readable for PricIo300Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io300::W`](W) writer structure"]
impl crate::Writable for PricIo300Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO300 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo300Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
