#[doc = "Register `PRIC_IO200` reader"]
pub type R = crate::R<PricIo200Spec>;
#[doc = "Register `PRIC_IO200` writer"]
pub type W = crate::W<PricIo200Spec>;
#[doc = "Field `EnblWrGroup0OfH2M` reader - Enable Write Group #0 of H2M"]
pub type EnblWrGroup0ofH2mR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfH2M` writer - Enable Write Group #0 of H2M"]
pub type EnblWrGroup0ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfH2M` reader - Enable Write Group #1 of H2M"]
pub type EnblWrGroup1ofH2mR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfH2M` writer - Enable Write Group #1 of H2M"]
pub type EnblWrGroup1ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfH2M` reader - Enable Write Group #2 of H2M"]
pub type EnblWrGroup2ofH2mR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfH2M` writer - Enable Write Group #2 of H2M"]
pub type EnblWrGroup2ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfH2M` reader - Enable Write Group #3 of H2M"]
pub type EnblWrGroup3ofH2mR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfH2M` writer - Enable Write Group #3 of H2M"]
pub type EnblWrGroup3ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfH2M` reader - Enable Write Group #4 of H2M"]
pub type EnblWrGroup4ofH2mR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfH2M` writer - Enable Write Group #4 of H2M"]
pub type EnblWrGroup4ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfH2M` reader - Enable Write Group #5 of H2M"]
pub type EnblWrGroup5ofH2mR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfH2M` writer - Enable Write Group #5 of H2M"]
pub type EnblWrGroup5ofH2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1200PRIC1_200\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1200pric12000500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1200pric12000500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1200pric12000500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1200PRIC12000500` reader - Enable Reset Tolerance of PRIC1200PRIC1_200\\[05:00\\]"]
pub type EnblRstToleranceOfPric1200pric12000500R =
    crate::BitReader<EnblRstToleranceOfPric1200pric12000500>;
impl EnblRstToleranceOfPric1200pric12000500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1200pric12000500 {
        match self.bits {
            false => EnblRstToleranceOfPric1200pric12000500::ResetBySrst,
            true => EnblRstToleranceOfPric1200pric12000500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1200pric12000500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1200pric12000500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1200PRIC12000500` writer - Enable Reset Tolerance of PRIC1200PRIC1_200\\[05:00\\]"]
pub type EnblRstToleranceOfPric1200pric12000500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1200pric12000500>;
impl<'a, REG> EnblRstToleranceOfPric1200pric12000500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1200pric12000500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1200pric12000500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1200PRIC12000600` reader - Enable Write Protection of PRIC1200PRIC1_200\\[06:00\\]"]
pub type EnblWrProtOfPric1200pric12000600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1200PRIC12000600` writer - Enable Write Protection of PRIC1200PRIC1_200\\[06:00\\]"]
pub type EnblWrProtOfPric1200pric12000600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfFMCMemory` reader - Enable Write Group #0 of FMC Memory"]
pub type EnblWrGroup0ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfFMCMemory` writer - Enable Write Group #0 of FMC Memory"]
pub type EnblWrGroup0ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfFMCMemory` reader - Enable Write Group #1 of FMC Memory"]
pub type EnblWrGroup1ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfFMCMemory` writer - Enable Write Group #1 of FMC Memory"]
pub type EnblWrGroup1ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfFMCMemory` reader - Enable Write Group #2 of FMC Memory"]
pub type EnblWrGroup2ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfFMCMemory` writer - Enable Write Group #2 of FMC Memory"]
pub type EnblWrGroup2ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfFMCMemory` reader - Enable Write Group #3 of FMC Memory"]
pub type EnblWrGroup3ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfFMCMemory` writer - Enable Write Group #3 of FMC Memory"]
pub type EnblWrGroup3ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfFMCMemory` reader - Enable Write Group #4 of FMC Memory"]
pub type EnblWrGroup4ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfFMCMemory` writer - Enable Write Group #4 of FMC Memory"]
pub type EnblWrGroup4ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfFMCMemory` reader - Enable Write Group #5 of FMC Memory"]
pub type EnblWrGroup5ofFmcmemoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfFMCMemory` writer - Enable Write Group #5 of FMC Memory"]
pub type EnblWrGroup5ofFmcmemoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1200PRIC1_200\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1200pric12001308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1200pric12001308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1200pric12001308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1200PRIC12001308` reader - Enable Reset Tolerance of PRIC1200PRIC1_200\\[13:08\\]"]
pub type EnblRstToleranceOfPric1200pric12001308R =
    crate::BitReader<EnblRstToleranceOfPric1200pric12001308>;
impl EnblRstToleranceOfPric1200pric12001308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1200pric12001308 {
        match self.bits {
            false => EnblRstToleranceOfPric1200pric12001308::ResetBySrst,
            true => EnblRstToleranceOfPric1200pric12001308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1200pric12001308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1200pric12001308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1200PRIC12001308` writer - Enable Reset Tolerance of PRIC1200PRIC1_200\\[13:08\\]"]
pub type EnblRstToleranceOfPric1200pric12001308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1200pric12001308>;
impl<'a, REG> EnblRstToleranceOfPric1200pric12001308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1200pric12001308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1200pric12001308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1200PRIC12001408` reader - Enable Write Protection of PRIC1200PRIC1_200\\[14:08\\]"]
pub type EnblWrProtOfPric1200pric12001408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1200PRIC12001408` writer - Enable Write Protection of PRIC1200PRIC1_200\\[14:08\\]"]
pub type EnblWrProtOfPric1200pric12001408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSPI0Memory` reader - Enable Write Group #0 of SPI0 Memory"]
pub type EnblWrGroup0ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSPI0Memory` writer - Enable Write Group #0 of SPI0 Memory"]
pub type EnblWrGroup0ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSPI0Memory` reader - Enable Write Group #1 of SPI0 Memory"]
pub type EnblWrGroup1ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSPI0Memory` writer - Enable Write Group #1 of SPI0 Memory"]
pub type EnblWrGroup1ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSPI0Memory` reader - Enable Write Group #2 of SPI0 Memory"]
pub type EnblWrGroup2ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSPI0Memory` writer - Enable Write Group #2 of SPI0 Memory"]
pub type EnblWrGroup2ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSPI0Memory` reader - Enable Write Group #3 of SPI0 Memory"]
pub type EnblWrGroup3ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSPI0Memory` writer - Enable Write Group #3 of SPI0 Memory"]
pub type EnblWrGroup3ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSPI0Memory` reader - Enable Write Group #4 of SPI0 Memory"]
pub type EnblWrGroup4ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSPI0Memory` writer - Enable Write Group #4 of SPI0 Memory"]
pub type EnblWrGroup4ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSPI0Memory` reader - Enable Write Group #5 of SPI0 Memory"]
pub type EnblWrGroup5ofSpi0memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSPI0Memory` writer - Enable Write Group #5 of SPI0 Memory"]
pub type EnblWrGroup5ofSpi0memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1200PRIC1_200\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1200pric12002116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1200pric12002116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1200pric12002116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1200PRIC12002116` reader - Enable Reset Tolerance of PRIC1200PRIC1_200\\[21:16\\]"]
pub type EnblRstToleranceOfPric1200pric12002116R =
    crate::BitReader<EnblRstToleranceOfPric1200pric12002116>;
impl EnblRstToleranceOfPric1200pric12002116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1200pric12002116 {
        match self.bits {
            false => EnblRstToleranceOfPric1200pric12002116::ResetBySrst,
            true => EnblRstToleranceOfPric1200pric12002116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1200pric12002116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1200pric12002116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1200PRIC12002116` writer - Enable Reset Tolerance of PRIC1200PRIC1_200\\[21:16\\]"]
pub type EnblRstToleranceOfPric1200pric12002116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1200pric12002116>;
impl<'a, REG> EnblRstToleranceOfPric1200pric12002116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1200pric12002116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1200pric12002116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1200PRIC12002216` reader - Enable Write Protection of PRIC1200PRIC1_200\\[22:16\\]"]
pub type EnblWrProtOfPric1200pric12002216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1200PRIC12002216` writer - Enable Write Protection of PRIC1200PRIC1_200\\[22:16\\]"]
pub type EnblWrProtOfPric1200pric12002216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSPI1Memory` reader - Enable Write Group #0 of SPI1 Memory"]
pub type EnblWrGroup0ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSPI1Memory` writer - Enable Write Group #0 of SPI1 Memory"]
pub type EnblWrGroup0ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSPI1Memory` reader - Enable Write Group #1 of SPI1 Memory"]
pub type EnblWrGroup1ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSPI1Memory` writer - Enable Write Group #1 of SPI1 Memory"]
pub type EnblWrGroup1ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSPI1Memory` reader - Enable Write Group #2 of SPI1 Memory"]
pub type EnblWrGroup2ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSPI1Memory` writer - Enable Write Group #2 of SPI1 Memory"]
pub type EnblWrGroup2ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSPI1Memory` reader - Enable Write Group #3 of SPI1 Memory"]
pub type EnblWrGroup3ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSPI1Memory` writer - Enable Write Group #3 of SPI1 Memory"]
pub type EnblWrGroup3ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSPI1Memory` reader - Enable Write Group #4 of SPI1 Memory"]
pub type EnblWrGroup4ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSPI1Memory` writer - Enable Write Group #4 of SPI1 Memory"]
pub type EnblWrGroup4ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSPI1Memory` reader - Enable Write Group #5 of SPI1 Memory"]
pub type EnblWrGroup5ofSpi1memoryR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSPI1Memory` writer - Enable Write Group #5 of SPI1 Memory"]
pub type EnblWrGroup5ofSpi1memoryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1200PRIC1_200\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1200pric12002924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1200pric12002924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1200pric12002924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1200PRIC12002924` reader - Enable Reset Tolerance of PRIC1200PRIC1_200\\[29:24\\]"]
pub type EnblRstToleranceOfPric1200pric12002924R =
    crate::BitReader<EnblRstToleranceOfPric1200pric12002924>;
impl EnblRstToleranceOfPric1200pric12002924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1200pric12002924 {
        match self.bits {
            false => EnblRstToleranceOfPric1200pric12002924::ResetBySrst,
            true => EnblRstToleranceOfPric1200pric12002924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1200pric12002924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1200pric12002924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1200PRIC12002924` writer - Enable Reset Tolerance of PRIC1200PRIC1_200\\[29:24\\]"]
pub type EnblRstToleranceOfPric1200pric12002924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1200pric12002924>;
impl<'a, REG> EnblRstToleranceOfPric1200pric12002924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1200pric12002924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1200pric12002924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1200PRIC12003024` reader - Enable Write Protection of PRIC1200PRIC1_200\\[30:24\\]"]
pub type EnblWrProtOfPric1200pric12003024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1200PRIC12003024` writer - Enable Write Protection of PRIC1200PRIC1_200\\[30:24\\]"]
pub type EnblWrProtOfPric1200pric12003024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group0of_h2m(&self) -> EnblWrGroup0ofH2mR {
        EnblWrGroup0ofH2mR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group1of_h2m(&self) -> EnblWrGroup1ofH2mR {
        EnblWrGroup1ofH2mR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group2of_h2m(&self) -> EnblWrGroup2ofH2mR {
        EnblWrGroup2ofH2mR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group3of_h2m(&self) -> EnblWrGroup3ofH2mR {
        EnblWrGroup3ofH2mR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group4of_h2m(&self) -> EnblWrGroup4ofH2mR {
        EnblWrGroup4ofH2mR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group5of_h2m(&self) -> EnblWrGroup5ofH2mR {
        EnblWrGroup5ofH2mR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1200PRIC1_200\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1200pric12000500(
        &self,
    ) -> EnblRstToleranceOfPric1200pric12000500R {
        EnblRstToleranceOfPric1200pric12000500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1200PRIC1_200\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1200pric12000600(&self) -> EnblWrProtOfPric1200pric12000600R {
        EnblWrProtOfPric1200pric12000600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group0of_fmcmemory(&self) -> EnblWrGroup0ofFmcmemoryR {
        EnblWrGroup0ofFmcmemoryR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group1of_fmcmemory(&self) -> EnblWrGroup1ofFmcmemoryR {
        EnblWrGroup1ofFmcmemoryR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group2of_fmcmemory(&self) -> EnblWrGroup2ofFmcmemoryR {
        EnblWrGroup2ofFmcmemoryR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group3of_fmcmemory(&self) -> EnblWrGroup3ofFmcmemoryR {
        EnblWrGroup3ofFmcmemoryR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group4of_fmcmemory(&self) -> EnblWrGroup4ofFmcmemoryR {
        EnblWrGroup4ofFmcmemoryR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group5of_fmcmemory(&self) -> EnblWrGroup5ofFmcmemoryR {
        EnblWrGroup5ofFmcmemoryR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1200PRIC1_200\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1200pric12001308(
        &self,
    ) -> EnblRstToleranceOfPric1200pric12001308R {
        EnblRstToleranceOfPric1200pric12001308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1200PRIC1_200\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1200pric12001408(&self) -> EnblWrProtOfPric1200pric12001408R {
        EnblWrProtOfPric1200pric12001408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi0memory(&self) -> EnblWrGroup0ofSpi0memoryR {
        EnblWrGroup0ofSpi0memoryR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi0memory(&self) -> EnblWrGroup1ofSpi0memoryR {
        EnblWrGroup1ofSpi0memoryR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi0memory(&self) -> EnblWrGroup2ofSpi0memoryR {
        EnblWrGroup2ofSpi0memoryR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi0memory(&self) -> EnblWrGroup3ofSpi0memoryR {
        EnblWrGroup3ofSpi0memoryR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi0memory(&self) -> EnblWrGroup4ofSpi0memoryR {
        EnblWrGroup4ofSpi0memoryR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi0memory(&self) -> EnblWrGroup5ofSpi0memoryR {
        EnblWrGroup5ofSpi0memoryR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1200PRIC1_200\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1200pric12002116(
        &self,
    ) -> EnblRstToleranceOfPric1200pric12002116R {
        EnblRstToleranceOfPric1200pric12002116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1200PRIC1_200\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1200pric12002216(&self) -> EnblWrProtOfPric1200pric12002216R {
        EnblWrProtOfPric1200pric12002216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi1memory(&self) -> EnblWrGroup0ofSpi1memoryR {
        EnblWrGroup0ofSpi1memoryR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi1memory(&self) -> EnblWrGroup1ofSpi1memoryR {
        EnblWrGroup1ofSpi1memoryR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi1memory(&self) -> EnblWrGroup2ofSpi1memoryR {
        EnblWrGroup2ofSpi1memoryR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi1memory(&self) -> EnblWrGroup3ofSpi1memoryR {
        EnblWrGroup3ofSpi1memoryR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi1memory(&self) -> EnblWrGroup4ofSpi1memoryR {
        EnblWrGroup4ofSpi1memoryR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi1memory(&self) -> EnblWrGroup5ofSpi1memoryR {
        EnblWrGroup5ofSpi1memoryR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1200PRIC1_200\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1200pric12002924(
        &self,
    ) -> EnblRstToleranceOfPric1200pric12002924R {
        EnblRstToleranceOfPric1200pric12002924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1200PRIC1_200\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1200pric12003024(&self) -> EnblWrProtOfPric1200pric12003024R {
        EnblWrProtOfPric1200pric12003024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group0of_h2m(&mut self) -> EnblWrGroup0ofH2mW<PricIo200Spec> {
        EnblWrGroup0ofH2mW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group1of_h2m(&mut self) -> EnblWrGroup1ofH2mW<PricIo200Spec> {
        EnblWrGroup1ofH2mW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group2of_h2m(&mut self) -> EnblWrGroup2ofH2mW<PricIo200Spec> {
        EnblWrGroup2ofH2mW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group3of_h2m(&mut self) -> EnblWrGroup3ofH2mW<PricIo200Spec> {
        EnblWrGroup3ofH2mW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group4of_h2m(&mut self) -> EnblWrGroup4ofH2mW<PricIo200Spec> {
        EnblWrGroup4ofH2mW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of H2M"]
    #[inline(always)]
    pub fn enbl_wr_group5of_h2m(&mut self) -> EnblWrGroup5ofH2mW<PricIo200Spec> {
        EnblWrGroup5ofH2mW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1200PRIC1_200\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1200pric12000500(
        &mut self,
    ) -> EnblRstToleranceOfPric1200pric12000500W<PricIo200Spec> {
        EnblRstToleranceOfPric1200pric12000500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1200PRIC1_200\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1200pric12000600(
        &mut self,
    ) -> EnblWrProtOfPric1200pric12000600W<PricIo200Spec> {
        EnblWrProtOfPric1200pric12000600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group0of_fmcmemory(&mut self) -> EnblWrGroup0ofFmcmemoryW<PricIo200Spec> {
        EnblWrGroup0ofFmcmemoryW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group1of_fmcmemory(&mut self) -> EnblWrGroup1ofFmcmemoryW<PricIo200Spec> {
        EnblWrGroup1ofFmcmemoryW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group2of_fmcmemory(&mut self) -> EnblWrGroup2ofFmcmemoryW<PricIo200Spec> {
        EnblWrGroup2ofFmcmemoryW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group3of_fmcmemory(&mut self) -> EnblWrGroup3ofFmcmemoryW<PricIo200Spec> {
        EnblWrGroup3ofFmcmemoryW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group4of_fmcmemory(&mut self) -> EnblWrGroup4ofFmcmemoryW<PricIo200Spec> {
        EnblWrGroup4ofFmcmemoryW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of FMC Memory"]
    #[inline(always)]
    pub fn enbl_wr_group5of_fmcmemory(&mut self) -> EnblWrGroup5ofFmcmemoryW<PricIo200Spec> {
        EnblWrGroup5ofFmcmemoryW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1200PRIC1_200\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1200pric12001308(
        &mut self,
    ) -> EnblRstToleranceOfPric1200pric12001308W<PricIo200Spec> {
        EnblRstToleranceOfPric1200pric12001308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1200PRIC1_200\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1200pric12001408(
        &mut self,
    ) -> EnblWrProtOfPric1200pric12001408W<PricIo200Spec> {
        EnblWrProtOfPric1200pric12001408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi0memory(&mut self) -> EnblWrGroup0ofSpi0memoryW<PricIo200Spec> {
        EnblWrGroup0ofSpi0memoryW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi0memory(&mut self) -> EnblWrGroup1ofSpi0memoryW<PricIo200Spec> {
        EnblWrGroup1ofSpi0memoryW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi0memory(&mut self) -> EnblWrGroup2ofSpi0memoryW<PricIo200Spec> {
        EnblWrGroup2ofSpi0memoryW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi0memory(&mut self) -> EnblWrGroup3ofSpi0memoryW<PricIo200Spec> {
        EnblWrGroup3ofSpi0memoryW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi0memory(&mut self) -> EnblWrGroup4ofSpi0memoryW<PricIo200Spec> {
        EnblWrGroup4ofSpi0memoryW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of SPI0 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi0memory(&mut self) -> EnblWrGroup5ofSpi0memoryW<PricIo200Spec> {
        EnblWrGroup5ofSpi0memoryW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1200PRIC1_200\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1200pric12002116(
        &mut self,
    ) -> EnblRstToleranceOfPric1200pric12002116W<PricIo200Spec> {
        EnblRstToleranceOfPric1200pric12002116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1200PRIC1_200\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1200pric12002216(
        &mut self,
    ) -> EnblWrProtOfPric1200pric12002216W<PricIo200Spec> {
        EnblWrProtOfPric1200pric12002216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi1memory(&mut self) -> EnblWrGroup0ofSpi1memoryW<PricIo200Spec> {
        EnblWrGroup0ofSpi1memoryW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi1memory(&mut self) -> EnblWrGroup1ofSpi1memoryW<PricIo200Spec> {
        EnblWrGroup1ofSpi1memoryW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi1memory(&mut self) -> EnblWrGroup2ofSpi1memoryW<PricIo200Spec> {
        EnblWrGroup2ofSpi1memoryW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi1memory(&mut self) -> EnblWrGroup3ofSpi1memoryW<PricIo200Spec> {
        EnblWrGroup3ofSpi1memoryW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi1memory(&mut self) -> EnblWrGroup4ofSpi1memoryW<PricIo200Spec> {
        EnblWrGroup4ofSpi1memoryW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of SPI1 Memory"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi1memory(&mut self) -> EnblWrGroup5ofSpi1memoryW<PricIo200Spec> {
        EnblWrGroup5ofSpi1memoryW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1200PRIC1_200\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1200pric12002924(
        &mut self,
    ) -> EnblRstToleranceOfPric1200pric12002924W<PricIo200Spec> {
        EnblRstToleranceOfPric1200pric12002924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1200PRIC1_200\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1200pric12003024(
        &mut self,
    ) -> EnblWrProtOfPric1200pric12003024W<PricIo200Spec> {
        EnblWrProtOfPric1200pric12003024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io200::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io200::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo200Spec;
impl crate::RegisterSpec for PricIo200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io200::R`](R) reader structure"]
impl crate::Readable for PricIo200Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io200::W`](W) writer structure"]
impl crate::Writable for PricIo200Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO200 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo200Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
