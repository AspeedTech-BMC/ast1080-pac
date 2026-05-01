#[doc = "Register `PRIC_IO004` reader"]
pub type R = crate::R<PricIo004Spec>;
#[doc = "Register `PRIC_IO004` writer"]
pub type W = crate::W<PricIo004Spec>;
#[doc = "Field `EnblWrGroup0OfFMCDMAAccess` reader - Enable Write Group #0 of FMC DMA access"]
pub type EnblWrGroup0ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfFMCDMAAccess` writer - Enable Write Group #0 of FMC DMA access"]
pub type EnblWrGroup0ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfFMCDMAAccess` reader - Enable Write Group #1 of FMC DMA access"]
pub type EnblWrGroup1ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfFMCDMAAccess` writer - Enable Write Group #1 of FMC DMA access"]
pub type EnblWrGroup1ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfFMCDMAAccess` reader - Enable Write Group #2 of FMC DMA access"]
pub type EnblWrGroup2ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfFMCDMAAccess` writer - Enable Write Group #2 of FMC DMA access"]
pub type EnblWrGroup2ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfFMCDMAAccess` reader - Enable Write Group #3 of FMC DMA access"]
pub type EnblWrGroup3ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfFMCDMAAccess` writer - Enable Write Group #3 of FMC DMA access"]
pub type EnblWrGroup3ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfFMCDMAAccess` reader - Enable Write Group #4 of FMC DMA access"]
pub type EnblWrGroup4ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfFMCDMAAccess` writer - Enable Write Group #4 of FMC DMA access"]
pub type EnblWrGroup4ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfFMCDMAAccess` reader - Enable Write Group #5 of FMC DMA access"]
pub type EnblWrGroup5ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfFMCDMAAccess` writer - Enable Write Group #5 of FMC DMA access"]
pub type EnblWrGroup5ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1004PRIC1_004\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1004pric10040500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1004pric10040500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1004pric10040500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1004PRIC10040500` reader - Enable Reset Tolerance of PRIC1004PRIC1_004\\[05:00\\]"]
pub type EnblRstToleranceOfPric1004pric10040500R =
    crate::BitReader<EnblRstToleranceOfPric1004pric10040500>;
impl EnblRstToleranceOfPric1004pric10040500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1004pric10040500 {
        match self.bits {
            false => EnblRstToleranceOfPric1004pric10040500::ResetBySrst,
            true => EnblRstToleranceOfPric1004pric10040500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1004pric10040500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1004pric10040500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1004PRIC10040500` writer - Enable Reset Tolerance of PRIC1004PRIC1_004\\[05:00\\]"]
pub type EnblRstToleranceOfPric1004pric10040500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1004pric10040500>;
impl<'a, REG> EnblRstToleranceOfPric1004pric10040500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1004pric10040500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1004pric10040500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1004PRIC10040600` reader - Enable Write Protection of PRIC1004PRIC1_004\\[06:00\\]"]
pub type EnblWrProtOfPric1004pric10040600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1004PRIC10040600` writer - Enable Write Protection of PRIC1004PRIC1_004\\[06:00\\]"]
pub type EnblWrProtOfPric1004pric10040600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfAHBMonitoringAccess` reader - Enable Write Group #0 of AHB Monitoring access"]
pub type EnblWrGroup0ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfAHBMonitoringAccess` writer - Enable Write Group #0 of AHB Monitoring access"]
pub type EnblWrGroup0ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfAHBMonitoringAccess` reader - Enable Write Group #1 of AHB Monitoring access"]
pub type EnblWrGroup1ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfAHBMonitoringAccess` writer - Enable Write Group #1 of AHB Monitoring access"]
pub type EnblWrGroup1ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfAHBMonitoringAccess` reader - Enable Write Group #2 of AHB Monitoring access"]
pub type EnblWrGroup2ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfAHBMonitoringAccess` writer - Enable Write Group #2 of AHB Monitoring access"]
pub type EnblWrGroup2ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfAHBMonitoringAccess` reader - Enable Write Group #3 of AHB Monitoring access"]
pub type EnblWrGroup3ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfAHBMonitoringAccess` writer - Enable Write Group #3 of AHB Monitoring access"]
pub type EnblWrGroup3ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfAHBMonitoringAccess` reader - Enable Write Group #4 of AHB Monitoring access"]
pub type EnblWrGroup4ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfAHBMonitoringAccess` writer - Enable Write Group #4 of AHB Monitoring access"]
pub type EnblWrGroup4ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfAHBMonitoringAccess` reader - Enable Write Group #5 of AHB Monitoring access"]
pub type EnblWrGroup5ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfAHBMonitoringAccess` writer - Enable Write Group #5 of AHB Monitoring access"]
pub type EnblWrGroup5ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1004PRIC1_004\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1004pric10041308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1004pric10041308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1004pric10041308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1004PRIC10041308` reader - Enable Reset Tolerance of PRIC1004PRIC1_004\\[13:08\\]"]
pub type EnblRstToleranceOfPric1004pric10041308R =
    crate::BitReader<EnblRstToleranceOfPric1004pric10041308>;
impl EnblRstToleranceOfPric1004pric10041308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1004pric10041308 {
        match self.bits {
            false => EnblRstToleranceOfPric1004pric10041308::ResetBySrst,
            true => EnblRstToleranceOfPric1004pric10041308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1004pric10041308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1004pric10041308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1004PRIC10041308` writer - Enable Reset Tolerance of PRIC1004PRIC1_004\\[13:08\\]"]
pub type EnblRstToleranceOfPric1004pric10041308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1004pric10041308>;
impl<'a, REG> EnblRstToleranceOfPric1004pric10041308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1004pric10041308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1004pric10041308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1004PRIC10041408` reader - Enable Write Protection of PRIC1004PRIC1_004\\[14:08\\]"]
pub type EnblWrProtOfPric1004pric10041408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1004PRIC10041408` writer - Enable Write Protection of PRIC1004PRIC1_004\\[14:08\\]"]
pub type EnblWrProtOfPric1004pric10041408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSPI0DMAAccess` reader - Enable Write Group #0 of SPI0 DMA access"]
pub type EnblWrGroup0ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSPI0DMAAccess` writer - Enable Write Group #0 of SPI0 DMA access"]
pub type EnblWrGroup0ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSPI0DMAAccess` reader - Enable Write Group #1 of SPI0 DMA access"]
pub type EnblWrGroup1ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSPI0DMAAccess` writer - Enable Write Group #1 of SPI0 DMA access"]
pub type EnblWrGroup1ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSPI0DMAAccess` reader - Enable Write Group #2 of SPI0 DMA access"]
pub type EnblWrGroup2ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSPI0DMAAccess` writer - Enable Write Group #2 of SPI0 DMA access"]
pub type EnblWrGroup2ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSPI0DMAAccess` reader - Enable Write Group #3 of SPI0 DMA access"]
pub type EnblWrGroup3ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSPI0DMAAccess` writer - Enable Write Group #3 of SPI0 DMA access"]
pub type EnblWrGroup3ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSPI0DMAAccess` reader - Enable Write Group #4 of SPI0 DMA access"]
pub type EnblWrGroup4ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSPI0DMAAccess` writer - Enable Write Group #4 of SPI0 DMA access"]
pub type EnblWrGroup4ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSPI0DMAAccess` reader - Enable Write Group #5 of SPI0 DMA access"]
pub type EnblWrGroup5ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSPI0DMAAccess` writer - Enable Write Group #5 of SPI0 DMA access"]
pub type EnblWrGroup5ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1004PRIC1_004\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1004pric10042116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1004pric10042116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1004pric10042116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1004PRIC10042116` reader - Enable Reset Tolerance of PRIC1004PRIC1_004\\[21:16\\]"]
pub type EnblRstToleranceOfPric1004pric10042116R =
    crate::BitReader<EnblRstToleranceOfPric1004pric10042116>;
impl EnblRstToleranceOfPric1004pric10042116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1004pric10042116 {
        match self.bits {
            false => EnblRstToleranceOfPric1004pric10042116::ResetBySrst,
            true => EnblRstToleranceOfPric1004pric10042116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1004pric10042116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1004pric10042116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1004PRIC10042116` writer - Enable Reset Tolerance of PRIC1004PRIC1_004\\[21:16\\]"]
pub type EnblRstToleranceOfPric1004pric10042116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1004pric10042116>;
impl<'a, REG> EnblRstToleranceOfPric1004pric10042116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1004pric10042116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1004pric10042116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1004PRIC10042216` reader - Enable Write Protection of PRIC1004PRIC1_004\\[22:16\\]"]
pub type EnblWrProtOfPric1004pric10042216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1004PRIC10042216` writer - Enable Write Protection of PRIC1004PRIC1_004\\[22:16\\]"]
pub type EnblWrProtOfPric1004pric10042216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSPI1DMAAccess` reader - Enable Write Group #0 of SPI1 DMA access"]
pub type EnblWrGroup0ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSPI1DMAAccess` writer - Enable Write Group #0 of SPI1 DMA access"]
pub type EnblWrGroup0ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSPI1DMAAccess` reader - Enable Write Group #1 of SPI1 DMA access"]
pub type EnblWrGroup1ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSPI1DMAAccess` writer - Enable Write Group #1 of SPI1 DMA access"]
pub type EnblWrGroup1ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSPI1DMAAccess` reader - Enable Write Group #2 of SPI1 DMA access"]
pub type EnblWrGroup2ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSPI1DMAAccess` writer - Enable Write Group #2 of SPI1 DMA access"]
pub type EnblWrGroup2ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSPI1DMAAccess` reader - Enable Write Group #3 of SPI1 DMA access"]
pub type EnblWrGroup3ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSPI1DMAAccess` writer - Enable Write Group #3 of SPI1 DMA access"]
pub type EnblWrGroup3ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSPI1DMAAccess` reader - Enable Write Group #4 of SPI1 DMA access"]
pub type EnblWrGroup4ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSPI1DMAAccess` writer - Enable Write Group #4 of SPI1 DMA access"]
pub type EnblWrGroup4ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSPI1DMAAccess` reader - Enable Write Group #5 of SPI1 DMA access"]
pub type EnblWrGroup5ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSPI1DMAAccess` writer - Enable Write Group #5 of SPI1 DMA access"]
pub type EnblWrGroup5ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1004PRIC1_004\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1004pric10042924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1004pric10042924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1004pric10042924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1004PRIC10042924` reader - Enable Reset Tolerance of PRIC1004PRIC1_004\\[29:24\\]"]
pub type EnblRstToleranceOfPric1004pric10042924R =
    crate::BitReader<EnblRstToleranceOfPric1004pric10042924>;
impl EnblRstToleranceOfPric1004pric10042924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1004pric10042924 {
        match self.bits {
            false => EnblRstToleranceOfPric1004pric10042924::ResetBySrst,
            true => EnblRstToleranceOfPric1004pric10042924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1004pric10042924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1004pric10042924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1004PRIC10042924` writer - Enable Reset Tolerance of PRIC1004PRIC1_004\\[29:24\\]"]
pub type EnblRstToleranceOfPric1004pric10042924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1004pric10042924>;
impl<'a, REG> EnblRstToleranceOfPric1004pric10042924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1004pric10042924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1004pric10042924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1004PRIC10043024` reader - Enable Write Protection of PRIC1004PRIC1_004\\[30:24\\]"]
pub type EnblWrProtOfPric1004pric10043024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1004PRIC10043024` writer - Enable Write Protection of PRIC1004PRIC1_004\\[30:24\\]"]
pub type EnblWrProtOfPric1004pric10043024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_fmcdmaaccess(&self) -> EnblWrGroup0ofFmcdmaaccessR {
        EnblWrGroup0ofFmcdmaaccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_fmcdmaaccess(&self) -> EnblWrGroup1ofFmcdmaaccessR {
        EnblWrGroup1ofFmcdmaaccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_fmcdmaaccess(&self) -> EnblWrGroup2ofFmcdmaaccessR {
        EnblWrGroup2ofFmcdmaaccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_fmcdmaaccess(&self) -> EnblWrGroup3ofFmcdmaaccessR {
        EnblWrGroup3ofFmcdmaaccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_fmcdmaaccess(&self) -> EnblWrGroup4ofFmcdmaaccessR {
        EnblWrGroup4ofFmcdmaaccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_fmcdmaaccess(&self) -> EnblWrGroup5ofFmcdmaaccessR {
        EnblWrGroup5ofFmcdmaaccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1004PRIC1_004\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1004pric10040500(
        &self,
    ) -> EnblRstToleranceOfPric1004pric10040500R {
        EnblRstToleranceOfPric1004pric10040500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1004PRIC1_004\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1004pric10040600(&self) -> EnblWrProtOfPric1004pric10040600R {
        EnblWrProtOfPric1004pric10040600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_ahbmonitoring_access(&self) -> EnblWrGroup0ofAhbmonitoringAccessR {
        EnblWrGroup0ofAhbmonitoringAccessR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_ahbmonitoring_access(&self) -> EnblWrGroup1ofAhbmonitoringAccessR {
        EnblWrGroup1ofAhbmonitoringAccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_ahbmonitoring_access(&self) -> EnblWrGroup2ofAhbmonitoringAccessR {
        EnblWrGroup2ofAhbmonitoringAccessR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_ahbmonitoring_access(&self) -> EnblWrGroup3ofAhbmonitoringAccessR {
        EnblWrGroup3ofAhbmonitoringAccessR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_ahbmonitoring_access(&self) -> EnblWrGroup4ofAhbmonitoringAccessR {
        EnblWrGroup4ofAhbmonitoringAccessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_ahbmonitoring_access(&self) -> EnblWrGroup5ofAhbmonitoringAccessR {
        EnblWrGroup5ofAhbmonitoringAccessR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1004PRIC1_004\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1004pric10041308(
        &self,
    ) -> EnblRstToleranceOfPric1004pric10041308R {
        EnblRstToleranceOfPric1004pric10041308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1004PRIC1_004\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1004pric10041408(&self) -> EnblWrProtOfPric1004pric10041408R {
        EnblWrProtOfPric1004pric10041408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi0dmaaccess(&self) -> EnblWrGroup0ofSpi0dmaaccessR {
        EnblWrGroup0ofSpi0dmaaccessR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi0dmaaccess(&self) -> EnblWrGroup1ofSpi0dmaaccessR {
        EnblWrGroup1ofSpi0dmaaccessR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi0dmaaccess(&self) -> EnblWrGroup2ofSpi0dmaaccessR {
        EnblWrGroup2ofSpi0dmaaccessR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi0dmaaccess(&self) -> EnblWrGroup3ofSpi0dmaaccessR {
        EnblWrGroup3ofSpi0dmaaccessR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi0dmaaccess(&self) -> EnblWrGroup4ofSpi0dmaaccessR {
        EnblWrGroup4ofSpi0dmaaccessR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi0dmaaccess(&self) -> EnblWrGroup5ofSpi0dmaaccessR {
        EnblWrGroup5ofSpi0dmaaccessR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1004PRIC1_004\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1004pric10042116(
        &self,
    ) -> EnblRstToleranceOfPric1004pric10042116R {
        EnblRstToleranceOfPric1004pric10042116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1004PRIC1_004\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1004pric10042216(&self) -> EnblWrProtOfPric1004pric10042216R {
        EnblWrProtOfPric1004pric10042216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi1dmaaccess(&self) -> EnblWrGroup0ofSpi1dmaaccessR {
        EnblWrGroup0ofSpi1dmaaccessR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi1dmaaccess(&self) -> EnblWrGroup1ofSpi1dmaaccessR {
        EnblWrGroup1ofSpi1dmaaccessR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi1dmaaccess(&self) -> EnblWrGroup2ofSpi1dmaaccessR {
        EnblWrGroup2ofSpi1dmaaccessR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi1dmaaccess(&self) -> EnblWrGroup3ofSpi1dmaaccessR {
        EnblWrGroup3ofSpi1dmaaccessR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi1dmaaccess(&self) -> EnblWrGroup4ofSpi1dmaaccessR {
        EnblWrGroup4ofSpi1dmaaccessR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi1dmaaccess(&self) -> EnblWrGroup5ofSpi1dmaaccessR {
        EnblWrGroup5ofSpi1dmaaccessR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1004PRIC1_004\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1004pric10042924(
        &self,
    ) -> EnblRstToleranceOfPric1004pric10042924R {
        EnblRstToleranceOfPric1004pric10042924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1004PRIC1_004\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1004pric10043024(&self) -> EnblWrProtOfPric1004pric10043024R {
        EnblWrProtOfPric1004pric10043024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_fmcdmaaccess(&mut self) -> EnblWrGroup0ofFmcdmaaccessW<PricIo004Spec> {
        EnblWrGroup0ofFmcdmaaccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_fmcdmaaccess(&mut self) -> EnblWrGroup1ofFmcdmaaccessW<PricIo004Spec> {
        EnblWrGroup1ofFmcdmaaccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_fmcdmaaccess(&mut self) -> EnblWrGroup2ofFmcdmaaccessW<PricIo004Spec> {
        EnblWrGroup2ofFmcdmaaccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_fmcdmaaccess(&mut self) -> EnblWrGroup3ofFmcdmaaccessW<PricIo004Spec> {
        EnblWrGroup3ofFmcdmaaccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_fmcdmaaccess(&mut self) -> EnblWrGroup4ofFmcdmaaccessW<PricIo004Spec> {
        EnblWrGroup4ofFmcdmaaccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_fmcdmaaccess(&mut self) -> EnblWrGroup5ofFmcdmaaccessW<PricIo004Spec> {
        EnblWrGroup5ofFmcdmaaccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1004PRIC1_004\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1004pric10040500(
        &mut self,
    ) -> EnblRstToleranceOfPric1004pric10040500W<PricIo004Spec> {
        EnblRstToleranceOfPric1004pric10040500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1004PRIC1_004\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1004pric10040600(
        &mut self,
    ) -> EnblWrProtOfPric1004pric10040600W<PricIo004Spec> {
        EnblWrProtOfPric1004pric10040600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_ahbmonitoring_access(
        &mut self,
    ) -> EnblWrGroup0ofAhbmonitoringAccessW<PricIo004Spec> {
        EnblWrGroup0ofAhbmonitoringAccessW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_ahbmonitoring_access(
        &mut self,
    ) -> EnblWrGroup1ofAhbmonitoringAccessW<PricIo004Spec> {
        EnblWrGroup1ofAhbmonitoringAccessW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_ahbmonitoring_access(
        &mut self,
    ) -> EnblWrGroup2ofAhbmonitoringAccessW<PricIo004Spec> {
        EnblWrGroup2ofAhbmonitoringAccessW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_ahbmonitoring_access(
        &mut self,
    ) -> EnblWrGroup3ofAhbmonitoringAccessW<PricIo004Spec> {
        EnblWrGroup3ofAhbmonitoringAccessW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_ahbmonitoring_access(
        &mut self,
    ) -> EnblWrGroup4ofAhbmonitoringAccessW<PricIo004Spec> {
        EnblWrGroup4ofAhbmonitoringAccessW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_ahbmonitoring_access(
        &mut self,
    ) -> EnblWrGroup5ofAhbmonitoringAccessW<PricIo004Spec> {
        EnblWrGroup5ofAhbmonitoringAccessW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1004PRIC1_004\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1004pric10041308(
        &mut self,
    ) -> EnblRstToleranceOfPric1004pric10041308W<PricIo004Spec> {
        EnblRstToleranceOfPric1004pric10041308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1004PRIC1_004\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1004pric10041408(
        &mut self,
    ) -> EnblWrProtOfPric1004pric10041408W<PricIo004Spec> {
        EnblWrProtOfPric1004pric10041408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi0dmaaccess(
        &mut self,
    ) -> EnblWrGroup0ofSpi0dmaaccessW<PricIo004Spec> {
        EnblWrGroup0ofSpi0dmaaccessW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi0dmaaccess(
        &mut self,
    ) -> EnblWrGroup1ofSpi0dmaaccessW<PricIo004Spec> {
        EnblWrGroup1ofSpi0dmaaccessW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi0dmaaccess(
        &mut self,
    ) -> EnblWrGroup2ofSpi0dmaaccessW<PricIo004Spec> {
        EnblWrGroup2ofSpi0dmaaccessW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi0dmaaccess(
        &mut self,
    ) -> EnblWrGroup3ofSpi0dmaaccessW<PricIo004Spec> {
        EnblWrGroup3ofSpi0dmaaccessW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi0dmaaccess(
        &mut self,
    ) -> EnblWrGroup4ofSpi0dmaaccessW<PricIo004Spec> {
        EnblWrGroup4ofSpi0dmaaccessW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi0dmaaccess(
        &mut self,
    ) -> EnblWrGroup5ofSpi0dmaaccessW<PricIo004Spec> {
        EnblWrGroup5ofSpi0dmaaccessW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1004PRIC1_004\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1004pric10042116(
        &mut self,
    ) -> EnblRstToleranceOfPric1004pric10042116W<PricIo004Spec> {
        EnblRstToleranceOfPric1004pric10042116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1004PRIC1_004\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1004pric10042216(
        &mut self,
    ) -> EnblWrProtOfPric1004pric10042216W<PricIo004Spec> {
        EnblWrProtOfPric1004pric10042216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi1dmaaccess(
        &mut self,
    ) -> EnblWrGroup0ofSpi1dmaaccessW<PricIo004Spec> {
        EnblWrGroup0ofSpi1dmaaccessW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi1dmaaccess(
        &mut self,
    ) -> EnblWrGroup1ofSpi1dmaaccessW<PricIo004Spec> {
        EnblWrGroup1ofSpi1dmaaccessW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi1dmaaccess(
        &mut self,
    ) -> EnblWrGroup2ofSpi1dmaaccessW<PricIo004Spec> {
        EnblWrGroup2ofSpi1dmaaccessW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi1dmaaccess(
        &mut self,
    ) -> EnblWrGroup3ofSpi1dmaaccessW<PricIo004Spec> {
        EnblWrGroup3ofSpi1dmaaccessW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi1dmaaccess(
        &mut self,
    ) -> EnblWrGroup4ofSpi1dmaaccessW<PricIo004Spec> {
        EnblWrGroup4ofSpi1dmaaccessW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi1dmaaccess(
        &mut self,
    ) -> EnblWrGroup5ofSpi1dmaaccessW<PricIo004Spec> {
        EnblWrGroup5ofSpi1dmaaccessW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1004PRIC1_004\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1004pric10042924(
        &mut self,
    ) -> EnblRstToleranceOfPric1004pric10042924W<PricIo004Spec> {
        EnblRstToleranceOfPric1004pric10042924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1004PRIC1_004\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1004pric10043024(
        &mut self,
    ) -> EnblWrProtOfPric1004pric10043024W<PricIo004Spec> {
        EnblWrProtOfPric1004pric10043024W::new(self, 31)
    }
}
#[doc = "Master Write Group Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo004Spec;
impl crate::RegisterSpec for PricIo004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io004::R`](R) reader structure"]
impl crate::Readable for PricIo004Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io004::W`](W) writer structure"]
impl crate::Writable for PricIo004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO004 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo004Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
