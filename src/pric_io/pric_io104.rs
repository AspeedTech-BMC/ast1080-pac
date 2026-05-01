#[doc = "Register `PRIC_IO104` reader"]
pub type R = crate::R<PricIo104Spec>;
#[doc = "Register `PRIC_IO104` writer"]
pub type W = crate::W<PricIo104Spec>;
#[doc = "Field `EnblReadGroup0OfFMCDMAAccess` reader - Enable Read Group #0 of FMC DMA access"]
pub type EnblReadGroup0ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfFMCDMAAccess` writer - Enable Read Group #0 of FMC DMA access"]
pub type EnblReadGroup0ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfFMCDMAAccess` reader - Enable Read Group #1 of FMC DMA access"]
pub type EnblReadGroup1ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfFMCDMAAccess` writer - Enable Read Group #1 of FMC DMA access"]
pub type EnblReadGroup1ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfFMCDMAAccess` reader - Enable Read Group #2 of FMC DMA access"]
pub type EnblReadGroup2ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfFMCDMAAccess` writer - Enable Read Group #2 of FMC DMA access"]
pub type EnblReadGroup2ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfFMCDMAAccess` reader - Enable Read Group #3 of FMC DMA access"]
pub type EnblReadGroup3ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfFMCDMAAccess` writer - Enable Read Group #3 of FMC DMA access"]
pub type EnblReadGroup3ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfFMCDMAAccess` reader - Enable Read Group #4 of FMC DMA access"]
pub type EnblReadGroup4ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfFMCDMAAccess` writer - Enable Read Group #4 of FMC DMA access"]
pub type EnblReadGroup4ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfFMCDMAAccess` reader - Enable Read Group #5 of FMC DMA access"]
pub type EnblReadGroup5ofFmcdmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfFMCDMAAccess` writer - Enable Read Group #5 of FMC DMA access"]
pub type EnblReadGroup5ofFmcdmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1104PRIC1_104\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1104pric11040500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1104pric11040500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1104pric11040500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1104PRIC11040500` reader - Enable Reset Tolerance of PRIC1104PRIC1_104\\[05:00\\]"]
pub type EnblRstToleranceOfPric1104pric11040500R =
    crate::BitReader<EnblRstToleranceOfPric1104pric11040500>;
impl EnblRstToleranceOfPric1104pric11040500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1104pric11040500 {
        match self.bits {
            false => EnblRstToleranceOfPric1104pric11040500::ResetBySrst,
            true => EnblRstToleranceOfPric1104pric11040500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1104pric11040500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1104pric11040500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1104PRIC11040500` writer - Enable Reset Tolerance of PRIC1104PRIC1_104\\[05:00\\]"]
pub type EnblRstToleranceOfPric1104pric11040500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1104pric11040500>;
impl<'a, REG> EnblRstToleranceOfPric1104pric11040500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1104pric11040500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1104pric11040500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1104PRIC11040600` reader - Enable Write Protection of PRIC1104PRIC1_104\\[06:00\\]"]
pub type EnblWrProtOfPric1104pric11040600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1104PRIC11040600` writer - Enable Write Protection of PRIC1104PRIC1_104\\[06:00\\]"]
pub type EnblWrProtOfPric1104pric11040600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfAHBMonitoringAccess` reader - Enable Read Group #0 of AHB Monitoring access"]
pub type EnblReadGroup0ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfAHBMonitoringAccess` writer - Enable Read Group #0 of AHB Monitoring access"]
pub type EnblReadGroup0ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfAHBMonitoringAccess` reader - Enable Read Group #1 of AHB Monitoring access"]
pub type EnblReadGroup1ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfAHBMonitoringAccess` writer - Enable Read Group #1 of AHB Monitoring access"]
pub type EnblReadGroup1ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfAHBMonitoringAccess` reader - Enable Read Group #2 of AHB Monitoring access"]
pub type EnblReadGroup2ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfAHBMonitoringAccess` writer - Enable Read Group #2 of AHB Monitoring access"]
pub type EnblReadGroup2ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfAHBMonitoringAccess` reader - Enable Read Group #3 of AHB Monitoring access"]
pub type EnblReadGroup3ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfAHBMonitoringAccess` writer - Enable Read Group #3 of AHB Monitoring access"]
pub type EnblReadGroup3ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfAHBMonitoringAccess` reader - Enable Read Group #4 of AHB Monitoring access"]
pub type EnblReadGroup4ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfAHBMonitoringAccess` writer - Enable Read Group #4 of AHB Monitoring access"]
pub type EnblReadGroup4ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfAHBMonitoringAccess` reader - Enable Read Group #5 of AHB Monitoring access"]
pub type EnblReadGroup5ofAhbmonitoringAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfAHBMonitoringAccess` writer - Enable Read Group #5 of AHB Monitoring access"]
pub type EnblReadGroup5ofAhbmonitoringAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1104PRIC1_104\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1104pric11041308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1104pric11041308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1104pric11041308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1104PRIC11041308` reader - Enable Reset Tolerance of PRIC1104PRIC1_104\\[13:08\\]"]
pub type EnblRstToleranceOfPric1104pric11041308R =
    crate::BitReader<EnblRstToleranceOfPric1104pric11041308>;
impl EnblRstToleranceOfPric1104pric11041308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1104pric11041308 {
        match self.bits {
            false => EnblRstToleranceOfPric1104pric11041308::ResetBySrst,
            true => EnblRstToleranceOfPric1104pric11041308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1104pric11041308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1104pric11041308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1104PRIC11041308` writer - Enable Reset Tolerance of PRIC1104PRIC1_104\\[13:08\\]"]
pub type EnblRstToleranceOfPric1104pric11041308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1104pric11041308>;
impl<'a, REG> EnblRstToleranceOfPric1104pric11041308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1104pric11041308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1104pric11041308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1104PRIC11041408` reader - Enable Write Protection of PRIC1104PRIC1_104\\[14:08\\]"]
pub type EnblWrProtOfPric1104pric11041408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1104PRIC11041408` writer - Enable Write Protection of PRIC1104PRIC1_104\\[14:08\\]"]
pub type EnblWrProtOfPric1104pric11041408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSPI0DMAAccess` reader - Enable Read Group #0 of SPI0 DMA access"]
pub type EnblReadGroup0ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSPI0DMAAccess` writer - Enable Read Group #0 of SPI0 DMA access"]
pub type EnblReadGroup0ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSPI0DMAAccess` reader - Enable Read Group #1 of SPI0 DMA access"]
pub type EnblReadGroup1ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSPI0DMAAccess` writer - Enable Read Group #1 of SPI0 DMA access"]
pub type EnblReadGroup1ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSPI0DMAAccess` reader - Enable Read Group #2 of SPI0 DMA access"]
pub type EnblReadGroup2ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSPI0DMAAccess` writer - Enable Read Group #2 of SPI0 DMA access"]
pub type EnblReadGroup2ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSPI0DMAAccess` reader - Enable Read Group #3 of SPI0 DMA access"]
pub type EnblReadGroup3ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSPI0DMAAccess` writer - Enable Read Group #3 of SPI0 DMA access"]
pub type EnblReadGroup3ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSPI0DMAAccess` reader - Enable Read Group #4 of SPI0 DMA access"]
pub type EnblReadGroup4ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSPI0DMAAccess` writer - Enable Read Group #4 of SPI0 DMA access"]
pub type EnblReadGroup4ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSPI0DMAAccess` reader - Enable Read Group #5 of SPI0 DMA access"]
pub type EnblReadGroup5ofSpi0dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSPI0DMAAccess` writer - Enable Read Group #5 of SPI0 DMA access"]
pub type EnblReadGroup5ofSpi0dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1104PRIC1_104\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1104pric11042116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1104pric11042116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1104pric11042116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1104PRIC11042116` reader - Enable Reset Tolerance of PRIC1104PRIC1_104\\[21:16\\]"]
pub type EnblRstToleranceOfPric1104pric11042116R =
    crate::BitReader<EnblRstToleranceOfPric1104pric11042116>;
impl EnblRstToleranceOfPric1104pric11042116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1104pric11042116 {
        match self.bits {
            false => EnblRstToleranceOfPric1104pric11042116::ResetBySrst,
            true => EnblRstToleranceOfPric1104pric11042116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1104pric11042116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1104pric11042116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1104PRIC11042116` writer - Enable Reset Tolerance of PRIC1104PRIC1_104\\[21:16\\]"]
pub type EnblRstToleranceOfPric1104pric11042116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1104pric11042116>;
impl<'a, REG> EnblRstToleranceOfPric1104pric11042116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1104pric11042116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1104pric11042116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1104PRIC11042216` reader - Enable Write Protection of PRIC1104PRIC1_104\\[22:16\\]"]
pub type EnblWrProtOfPric1104pric11042216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1104PRIC11042216` writer - Enable Write Protection of PRIC1104PRIC1_104\\[22:16\\]"]
pub type EnblWrProtOfPric1104pric11042216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSPI1DMAAccess` reader - Enable Read Group #0 of SPI1 DMA access"]
pub type EnblReadGroup0ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSPI1DMAAccess` writer - Enable Read Group #0 of SPI1 DMA access"]
pub type EnblReadGroup0ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSPI1DMAAccess` reader - Enable Read Group #1 of SPI1 DMA access"]
pub type EnblReadGroup1ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSPI1DMAAccess` writer - Enable Read Group #1 of SPI1 DMA access"]
pub type EnblReadGroup1ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSPI1DMAAccess` reader - Enable Read Group #2 of SPI1 DMA access"]
pub type EnblReadGroup2ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSPI1DMAAccess` writer - Enable Read Group #2 of SPI1 DMA access"]
pub type EnblReadGroup2ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSPI1DMAAccess` reader - Enable Read Group #3 of SPI1 DMA access"]
pub type EnblReadGroup3ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSPI1DMAAccess` writer - Enable Read Group #3 of SPI1 DMA access"]
pub type EnblReadGroup3ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSPI1DMAAccess` reader - Enable Read Group #4 of SPI1 DMA access"]
pub type EnblReadGroup4ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSPI1DMAAccess` writer - Enable Read Group #4 of SPI1 DMA access"]
pub type EnblReadGroup4ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSPI1DMAAccess` reader - Enable Read Group #5 of SPI1 DMA access"]
pub type EnblReadGroup5ofSpi1dmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSPI1DMAAccess` writer - Enable Read Group #5 of SPI1 DMA access"]
pub type EnblReadGroup5ofSpi1dmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1104PRIC1_104\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1104pric11042924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1104pric11042924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1104pric11042924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1104PRIC11042924` reader - Enable Reset Tolerance of PRIC1104PRIC1_104\\[29:24\\]"]
pub type EnblRstToleranceOfPric1104pric11042924R =
    crate::BitReader<EnblRstToleranceOfPric1104pric11042924>;
impl EnblRstToleranceOfPric1104pric11042924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1104pric11042924 {
        match self.bits {
            false => EnblRstToleranceOfPric1104pric11042924::ResetBySrst,
            true => EnblRstToleranceOfPric1104pric11042924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1104pric11042924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1104pric11042924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1104PRIC11042924` writer - Enable Reset Tolerance of PRIC1104PRIC1_104\\[29:24\\]"]
pub type EnblRstToleranceOfPric1104pric11042924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1104pric11042924>;
impl<'a, REG> EnblRstToleranceOfPric1104pric11042924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1104pric11042924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1104pric11042924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1104PRIC11043024` reader - Enable Write Protection of PRIC1104PRIC1_104\\[30:24\\]"]
pub type EnblWrProtOfPric1104pric11043024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1104PRIC11043024` writer - Enable Write Protection of PRIC1104PRIC1_104\\[30:24\\]"]
pub type EnblWrProtOfPric1104pric11043024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group0of_fmcdmaaccess(&self) -> EnblReadGroup0ofFmcdmaaccessR {
        EnblReadGroup0ofFmcdmaaccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group1of_fmcdmaaccess(&self) -> EnblReadGroup1ofFmcdmaaccessR {
        EnblReadGroup1ofFmcdmaaccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group2of_fmcdmaaccess(&self) -> EnblReadGroup2ofFmcdmaaccessR {
        EnblReadGroup2ofFmcdmaaccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group3of_fmcdmaaccess(&self) -> EnblReadGroup3ofFmcdmaaccessR {
        EnblReadGroup3ofFmcdmaaccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group4of_fmcdmaaccess(&self) -> EnblReadGroup4ofFmcdmaaccessR {
        EnblReadGroup4ofFmcdmaaccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group5of_fmcdmaaccess(&self) -> EnblReadGroup5ofFmcdmaaccessR {
        EnblReadGroup5ofFmcdmaaccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1104PRIC1_104\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1104pric11040500(
        &self,
    ) -> EnblRstToleranceOfPric1104pric11040500R {
        EnblRstToleranceOfPric1104pric11040500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1104PRIC1_104\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1104pric11040600(&self) -> EnblWrProtOfPric1104pric11040600R {
        EnblWrProtOfPric1104pric11040600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group0of_ahbmonitoring_access(&self) -> EnblReadGroup0ofAhbmonitoringAccessR {
        EnblReadGroup0ofAhbmonitoringAccessR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group1of_ahbmonitoring_access(&self) -> EnblReadGroup1ofAhbmonitoringAccessR {
        EnblReadGroup1ofAhbmonitoringAccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group2of_ahbmonitoring_access(&self) -> EnblReadGroup2ofAhbmonitoringAccessR {
        EnblReadGroup2ofAhbmonitoringAccessR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group3of_ahbmonitoring_access(&self) -> EnblReadGroup3ofAhbmonitoringAccessR {
        EnblReadGroup3ofAhbmonitoringAccessR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group4of_ahbmonitoring_access(&self) -> EnblReadGroup4ofAhbmonitoringAccessR {
        EnblReadGroup4ofAhbmonitoringAccessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group5of_ahbmonitoring_access(&self) -> EnblReadGroup5ofAhbmonitoringAccessR {
        EnblReadGroup5ofAhbmonitoringAccessR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1104PRIC1_104\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1104pric11041308(
        &self,
    ) -> EnblRstToleranceOfPric1104pric11041308R {
        EnblRstToleranceOfPric1104pric11041308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1104PRIC1_104\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1104pric11041408(&self) -> EnblWrProtOfPric1104pric11041408R {
        EnblWrProtOfPric1104pric11041408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi0dmaaccess(&self) -> EnblReadGroup0ofSpi0dmaaccessR {
        EnblReadGroup0ofSpi0dmaaccessR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi0dmaaccess(&self) -> EnblReadGroup1ofSpi0dmaaccessR {
        EnblReadGroup1ofSpi0dmaaccessR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi0dmaaccess(&self) -> EnblReadGroup2ofSpi0dmaaccessR {
        EnblReadGroup2ofSpi0dmaaccessR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi0dmaaccess(&self) -> EnblReadGroup3ofSpi0dmaaccessR {
        EnblReadGroup3ofSpi0dmaaccessR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi0dmaaccess(&self) -> EnblReadGroup4ofSpi0dmaaccessR {
        EnblReadGroup4ofSpi0dmaaccessR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi0dmaaccess(&self) -> EnblReadGroup5ofSpi0dmaaccessR {
        EnblReadGroup5ofSpi0dmaaccessR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1104PRIC1_104\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1104pric11042116(
        &self,
    ) -> EnblRstToleranceOfPric1104pric11042116R {
        EnblRstToleranceOfPric1104pric11042116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1104PRIC1_104\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1104pric11042216(&self) -> EnblWrProtOfPric1104pric11042216R {
        EnblWrProtOfPric1104pric11042216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi1dmaaccess(&self) -> EnblReadGroup0ofSpi1dmaaccessR {
        EnblReadGroup0ofSpi1dmaaccessR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi1dmaaccess(&self) -> EnblReadGroup1ofSpi1dmaaccessR {
        EnblReadGroup1ofSpi1dmaaccessR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi1dmaaccess(&self) -> EnblReadGroup2ofSpi1dmaaccessR {
        EnblReadGroup2ofSpi1dmaaccessR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi1dmaaccess(&self) -> EnblReadGroup3ofSpi1dmaaccessR {
        EnblReadGroup3ofSpi1dmaaccessR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi1dmaaccess(&self) -> EnblReadGroup4ofSpi1dmaaccessR {
        EnblReadGroup4ofSpi1dmaaccessR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi1dmaaccess(&self) -> EnblReadGroup5ofSpi1dmaaccessR {
        EnblReadGroup5ofSpi1dmaaccessR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1104PRIC1_104\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1104pric11042924(
        &self,
    ) -> EnblRstToleranceOfPric1104pric11042924R {
        EnblRstToleranceOfPric1104pric11042924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1104PRIC1_104\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1104pric11043024(&self) -> EnblWrProtOfPric1104pric11043024R {
        EnblWrProtOfPric1104pric11043024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group0of_fmcdmaaccess(
        &mut self,
    ) -> EnblReadGroup0ofFmcdmaaccessW<PricIo104Spec> {
        EnblReadGroup0ofFmcdmaaccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group1of_fmcdmaaccess(
        &mut self,
    ) -> EnblReadGroup1ofFmcdmaaccessW<PricIo104Spec> {
        EnblReadGroup1ofFmcdmaaccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group2of_fmcdmaaccess(
        &mut self,
    ) -> EnblReadGroup2ofFmcdmaaccessW<PricIo104Spec> {
        EnblReadGroup2ofFmcdmaaccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group3of_fmcdmaaccess(
        &mut self,
    ) -> EnblReadGroup3ofFmcdmaaccessW<PricIo104Spec> {
        EnblReadGroup3ofFmcdmaaccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group4of_fmcdmaaccess(
        &mut self,
    ) -> EnblReadGroup4ofFmcdmaaccessW<PricIo104Spec> {
        EnblReadGroup4ofFmcdmaaccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of FMC DMA access"]
    #[inline(always)]
    pub fn enbl_read_group5of_fmcdmaaccess(
        &mut self,
    ) -> EnblReadGroup5ofFmcdmaaccessW<PricIo104Spec> {
        EnblReadGroup5ofFmcdmaaccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1104PRIC1_104\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1104pric11040500(
        &mut self,
    ) -> EnblRstToleranceOfPric1104pric11040500W<PricIo104Spec> {
        EnblRstToleranceOfPric1104pric11040500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1104PRIC1_104\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1104pric11040600(
        &mut self,
    ) -> EnblWrProtOfPric1104pric11040600W<PricIo104Spec> {
        EnblWrProtOfPric1104pric11040600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group0of_ahbmonitoring_access(
        &mut self,
    ) -> EnblReadGroup0ofAhbmonitoringAccessW<PricIo104Spec> {
        EnblReadGroup0ofAhbmonitoringAccessW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group1of_ahbmonitoring_access(
        &mut self,
    ) -> EnblReadGroup1ofAhbmonitoringAccessW<PricIo104Spec> {
        EnblReadGroup1ofAhbmonitoringAccessW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group2of_ahbmonitoring_access(
        &mut self,
    ) -> EnblReadGroup2ofAhbmonitoringAccessW<PricIo104Spec> {
        EnblReadGroup2ofAhbmonitoringAccessW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group3of_ahbmonitoring_access(
        &mut self,
    ) -> EnblReadGroup3ofAhbmonitoringAccessW<PricIo104Spec> {
        EnblReadGroup3ofAhbmonitoringAccessW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group4of_ahbmonitoring_access(
        &mut self,
    ) -> EnblReadGroup4ofAhbmonitoringAccessW<PricIo104Spec> {
        EnblReadGroup4ofAhbmonitoringAccessW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of AHB Monitoring access"]
    #[inline(always)]
    pub fn enbl_read_group5of_ahbmonitoring_access(
        &mut self,
    ) -> EnblReadGroup5ofAhbmonitoringAccessW<PricIo104Spec> {
        EnblReadGroup5ofAhbmonitoringAccessW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1104PRIC1_104\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1104pric11041308(
        &mut self,
    ) -> EnblRstToleranceOfPric1104pric11041308W<PricIo104Spec> {
        EnblRstToleranceOfPric1104pric11041308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1104PRIC1_104\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1104pric11041408(
        &mut self,
    ) -> EnblWrProtOfPric1104pric11041408W<PricIo104Spec> {
        EnblWrProtOfPric1104pric11041408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi0dmaaccess(
        &mut self,
    ) -> EnblReadGroup0ofSpi0dmaaccessW<PricIo104Spec> {
        EnblReadGroup0ofSpi0dmaaccessW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi0dmaaccess(
        &mut self,
    ) -> EnblReadGroup1ofSpi0dmaaccessW<PricIo104Spec> {
        EnblReadGroup1ofSpi0dmaaccessW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi0dmaaccess(
        &mut self,
    ) -> EnblReadGroup2ofSpi0dmaaccessW<PricIo104Spec> {
        EnblReadGroup2ofSpi0dmaaccessW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi0dmaaccess(
        &mut self,
    ) -> EnblReadGroup3ofSpi0dmaaccessW<PricIo104Spec> {
        EnblReadGroup3ofSpi0dmaaccessW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi0dmaaccess(
        &mut self,
    ) -> EnblReadGroup4ofSpi0dmaaccessW<PricIo104Spec> {
        EnblReadGroup4ofSpi0dmaaccessW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of SPI0 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi0dmaaccess(
        &mut self,
    ) -> EnblReadGroup5ofSpi0dmaaccessW<PricIo104Spec> {
        EnblReadGroup5ofSpi0dmaaccessW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1104PRIC1_104\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1104pric11042116(
        &mut self,
    ) -> EnblRstToleranceOfPric1104pric11042116W<PricIo104Spec> {
        EnblRstToleranceOfPric1104pric11042116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1104PRIC1_104\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1104pric11042216(
        &mut self,
    ) -> EnblWrProtOfPric1104pric11042216W<PricIo104Spec> {
        EnblWrProtOfPric1104pric11042216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi1dmaaccess(
        &mut self,
    ) -> EnblReadGroup0ofSpi1dmaaccessW<PricIo104Spec> {
        EnblReadGroup0ofSpi1dmaaccessW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi1dmaaccess(
        &mut self,
    ) -> EnblReadGroup1ofSpi1dmaaccessW<PricIo104Spec> {
        EnblReadGroup1ofSpi1dmaaccessW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi1dmaaccess(
        &mut self,
    ) -> EnblReadGroup2ofSpi1dmaaccessW<PricIo104Spec> {
        EnblReadGroup2ofSpi1dmaaccessW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi1dmaaccess(
        &mut self,
    ) -> EnblReadGroup3ofSpi1dmaaccessW<PricIo104Spec> {
        EnblReadGroup3ofSpi1dmaaccessW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi1dmaaccess(
        &mut self,
    ) -> EnblReadGroup4ofSpi1dmaaccessW<PricIo104Spec> {
        EnblReadGroup4ofSpi1dmaaccessW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of SPI1 DMA access"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi1dmaaccess(
        &mut self,
    ) -> EnblReadGroup5ofSpi1dmaaccessW<PricIo104Spec> {
        EnblReadGroup5ofSpi1dmaaccessW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1104PRIC1_104\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1104pric11042924(
        &mut self,
    ) -> EnblRstToleranceOfPric1104pric11042924W<PricIo104Spec> {
        EnblRstToleranceOfPric1104pric11042924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1104PRIC1_104\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1104pric11043024(
        &mut self,
    ) -> EnblWrProtOfPric1104pric11043024W<PricIo104Spec> {
        EnblWrProtOfPric1104pric11043024W::new(self, 31)
    }
}
#[doc = "Master Read Group Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo104Spec;
impl crate::RegisterSpec for PricIo104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io104::R`](R) reader structure"]
impl crate::Readable for PricIo104Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io104::W`](W) writer structure"]
impl crate::Writable for PricIo104Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO104 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo104Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
