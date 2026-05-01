#[doc = "Register `PRIC_IO204` reader"]
pub type R = crate::R<PricIo204Spec>;
#[doc = "Register `PRIC_IO204` writer"]
pub type W = crate::W<PricIo204Spec>;
#[doc = "Field `EnblWrGroup0OfFMCReg` reader - Enable Write Group #0 of FMC Register"]
pub type EnblWrGroup0ofFmcregR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfFMCReg` writer - Enable Write Group #0 of FMC Register"]
pub type EnblWrGroup0ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfFMCReg` reader - Enable Write Group #1 of FMC Register"]
pub type EnblWrGroup1ofFmcregR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfFMCReg` writer - Enable Write Group #1 of FMC Register"]
pub type EnblWrGroup1ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfFMCReg` reader - Enable Write Group #2 of FMC Register"]
pub type EnblWrGroup2ofFmcregR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfFMCReg` writer - Enable Write Group #2 of FMC Register"]
pub type EnblWrGroup2ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfFMCReg` reader - Enable Write Group #3 of FMC Register"]
pub type EnblWrGroup3ofFmcregR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfFMCReg` writer - Enable Write Group #3 of FMC Register"]
pub type EnblWrGroup3ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfFMCReg` reader - Enable Write Group #4 of FMC Register"]
pub type EnblWrGroup4ofFmcregR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfFMCReg` writer - Enable Write Group #4 of FMC Register"]
pub type EnblWrGroup4ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfFMCReg` reader - Enable Write Group #5 of FMC Register"]
pub type EnblWrGroup5ofFmcregR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfFMCReg` writer - Enable Write Group #5 of FMC Register"]
pub type EnblWrGroup5ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1204PRIC1_204\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1204pric12040500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1204pric12040500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1204pric12040500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1204PRIC12040500` reader - Enable Reset Tolerance of PRIC1204PRIC1_204\\[05:00\\]"]
pub type EnblRstToleranceOfPric1204pric12040500R =
    crate::BitReader<EnblRstToleranceOfPric1204pric12040500>;
impl EnblRstToleranceOfPric1204pric12040500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1204pric12040500 {
        match self.bits {
            false => EnblRstToleranceOfPric1204pric12040500::ResetBySrst,
            true => EnblRstToleranceOfPric1204pric12040500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1204pric12040500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1204pric12040500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1204PRIC12040500` writer - Enable Reset Tolerance of PRIC1204PRIC1_204\\[05:00\\]"]
pub type EnblRstToleranceOfPric1204pric12040500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1204pric12040500>;
impl<'a, REG> EnblRstToleranceOfPric1204pric12040500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1204pric12040500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1204pric12040500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1204PRIC12040600` reader - Enable Write Protection of PRIC1204PRIC1_204\\[06:00\\]"]
pub type EnblWrProtOfPric1204pric12040600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1204PRIC12040600` writer - Enable Write Protection of PRIC1204PRIC1_204\\[06:00\\]"]
pub type EnblWrProtOfPric1204pric12040600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSPI0Reg` reader - Enable Write Group #0 of SPI0 Register"]
pub type EnblWrGroup0ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSPI0Reg` writer - Enable Write Group #0 of SPI0 Register"]
pub type EnblWrGroup0ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSPI0Reg` reader - Enable Write Group #1 of SPI0 Register"]
pub type EnblWrGroup1ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSPI0Reg` writer - Enable Write Group #1 of SPI0 Register"]
pub type EnblWrGroup1ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSPI0Reg` reader - Enable Write Group #2 of SPI0 Register"]
pub type EnblWrGroup2ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSPI0Reg` writer - Enable Write Group #2 of SPI0 Register"]
pub type EnblWrGroup2ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSPI0Reg` reader - Enable Write Group #3 of SPI0 Register"]
pub type EnblWrGroup3ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSPI0Reg` writer - Enable Write Group #3 of SPI0 Register"]
pub type EnblWrGroup3ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSPI0Reg` reader - Enable Write Group #4 of SPI0 Register"]
pub type EnblWrGroup4ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSPI0Reg` writer - Enable Write Group #4 of SPI0 Register"]
pub type EnblWrGroup4ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSPI0Reg` reader - Enable Write Group #5 of SPI0 Register"]
pub type EnblWrGroup5ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSPI0Reg` writer - Enable Write Group #5 of SPI0 Register"]
pub type EnblWrGroup5ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1204PRIC1_204\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1204pric12041308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1204pric12041308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1204pric12041308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1204PRIC12041308` reader - Enable Reset Tolerance of PRIC1204PRIC1_204\\[13:08\\]"]
pub type EnblRstToleranceOfPric1204pric12041308R =
    crate::BitReader<EnblRstToleranceOfPric1204pric12041308>;
impl EnblRstToleranceOfPric1204pric12041308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1204pric12041308 {
        match self.bits {
            false => EnblRstToleranceOfPric1204pric12041308::ResetBySrst,
            true => EnblRstToleranceOfPric1204pric12041308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1204pric12041308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1204pric12041308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1204PRIC12041308` writer - Enable Reset Tolerance of PRIC1204PRIC1_204\\[13:08\\]"]
pub type EnblRstToleranceOfPric1204pric12041308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1204pric12041308>;
impl<'a, REG> EnblRstToleranceOfPric1204pric12041308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1204pric12041308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1204pric12041308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1204PRIC12041408` reader - Enable Write Protection of PRIC1204PRIC1_204\\[14:08\\]"]
pub type EnblWrProtOfPric1204pric12041408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1204PRIC12041408` writer - Enable Write Protection of PRIC1204PRIC1_204\\[14:08\\]"]
pub type EnblWrProtOfPric1204pric12041408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfSPI1Reg` reader - Enable Write Group #0 of SPI1 Register"]
pub type EnblWrGroup0ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfSPI1Reg` writer - Enable Write Group #0 of SPI1 Register"]
pub type EnblWrGroup0ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfSPI1Reg` reader - Enable Write Group #1 of SPI1 Register"]
pub type EnblWrGroup1ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfSPI1Reg` writer - Enable Write Group #1 of SPI1 Register"]
pub type EnblWrGroup1ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfSPI1Reg` reader - Enable Write Group #2 of SPI1 Register"]
pub type EnblWrGroup2ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfSPI1Reg` writer - Enable Write Group #2 of SPI1 Register"]
pub type EnblWrGroup2ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfSPI1Reg` reader - Enable Write Group #3 of SPI1 Register"]
pub type EnblWrGroup3ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfSPI1Reg` writer - Enable Write Group #3 of SPI1 Register"]
pub type EnblWrGroup3ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfSPI1Reg` reader - Enable Write Group #4 of SPI1 Register"]
pub type EnblWrGroup4ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfSPI1Reg` writer - Enable Write Group #4 of SPI1 Register"]
pub type EnblWrGroup4ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfSPI1Reg` reader - Enable Write Group #5 of SPI1 Register"]
pub type EnblWrGroup5ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfSPI1Reg` writer - Enable Write Group #5 of SPI1 Register"]
pub type EnblWrGroup5ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1204PRIC1_204\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1204pric12042116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1204pric12042116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1204pric12042116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1204PRIC12042116` reader - Enable Reset Tolerance of PRIC1204PRIC1_204\\[21:16\\]"]
pub type EnblRstToleranceOfPric1204pric12042116R =
    crate::BitReader<EnblRstToleranceOfPric1204pric12042116>;
impl EnblRstToleranceOfPric1204pric12042116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1204pric12042116 {
        match self.bits {
            false => EnblRstToleranceOfPric1204pric12042116::ResetBySrst,
            true => EnblRstToleranceOfPric1204pric12042116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1204pric12042116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1204pric12042116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1204PRIC12042116` writer - Enable Reset Tolerance of PRIC1204PRIC1_204\\[21:16\\]"]
pub type EnblRstToleranceOfPric1204pric12042116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1204pric12042116>;
impl<'a, REG> EnblRstToleranceOfPric1204pric12042116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1204pric12042116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1204pric12042116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1204PRIC12042216` reader - Enable Write Protection of PRIC1204PRIC1_204\\[22:16\\]"]
pub type EnblWrProtOfPric1204pric12042216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1204PRIC12042216` writer - Enable Write Protection of PRIC1204PRIC1_204\\[22:16\\]"]
pub type EnblWrProtOfPric1204pric12042216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group0of_fmcreg(&self) -> EnblWrGroup0ofFmcregR {
        EnblWrGroup0ofFmcregR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group1of_fmcreg(&self) -> EnblWrGroup1ofFmcregR {
        EnblWrGroup1ofFmcregR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group2of_fmcreg(&self) -> EnblWrGroup2ofFmcregR {
        EnblWrGroup2ofFmcregR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group3of_fmcreg(&self) -> EnblWrGroup3ofFmcregR {
        EnblWrGroup3ofFmcregR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group4of_fmcreg(&self) -> EnblWrGroup4ofFmcregR {
        EnblWrGroup4ofFmcregR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group5of_fmcreg(&self) -> EnblWrGroup5ofFmcregR {
        EnblWrGroup5ofFmcregR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1204PRIC1_204\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1204pric12040500(
        &self,
    ) -> EnblRstToleranceOfPric1204pric12040500R {
        EnblRstToleranceOfPric1204pric12040500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1204PRIC1_204\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1204pric12040600(&self) -> EnblWrProtOfPric1204pric12040600R {
        EnblWrProtOfPric1204pric12040600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi0reg(&self) -> EnblWrGroup0ofSpi0regR {
        EnblWrGroup0ofSpi0regR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi0reg(&self) -> EnblWrGroup1ofSpi0regR {
        EnblWrGroup1ofSpi0regR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi0reg(&self) -> EnblWrGroup2ofSpi0regR {
        EnblWrGroup2ofSpi0regR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi0reg(&self) -> EnblWrGroup3ofSpi0regR {
        EnblWrGroup3ofSpi0regR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi0reg(&self) -> EnblWrGroup4ofSpi0regR {
        EnblWrGroup4ofSpi0regR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi0reg(&self) -> EnblWrGroup5ofSpi0regR {
        EnblWrGroup5ofSpi0regR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1204PRIC1_204\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1204pric12041308(
        &self,
    ) -> EnblRstToleranceOfPric1204pric12041308R {
        EnblRstToleranceOfPric1204pric12041308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1204PRIC1_204\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1204pric12041408(&self) -> EnblWrProtOfPric1204pric12041408R {
        EnblWrProtOfPric1204pric12041408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi1reg(&self) -> EnblWrGroup0ofSpi1regR {
        EnblWrGroup0ofSpi1regR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi1reg(&self) -> EnblWrGroup1ofSpi1regR {
        EnblWrGroup1ofSpi1regR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi1reg(&self) -> EnblWrGroup2ofSpi1regR {
        EnblWrGroup2ofSpi1regR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi1reg(&self) -> EnblWrGroup3ofSpi1regR {
        EnblWrGroup3ofSpi1regR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi1reg(&self) -> EnblWrGroup4ofSpi1regR {
        EnblWrGroup4ofSpi1regR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi1reg(&self) -> EnblWrGroup5ofSpi1regR {
        EnblWrGroup5ofSpi1regR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1204PRIC1_204\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1204pric12042116(
        &self,
    ) -> EnblRstToleranceOfPric1204pric12042116R {
        EnblRstToleranceOfPric1204pric12042116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1204PRIC1_204\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1204pric12042216(&self) -> EnblWrProtOfPric1204pric12042216R {
        EnblWrProtOfPric1204pric12042216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group0of_fmcreg(&mut self) -> EnblWrGroup0ofFmcregW<PricIo204Spec> {
        EnblWrGroup0ofFmcregW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group1of_fmcreg(&mut self) -> EnblWrGroup1ofFmcregW<PricIo204Spec> {
        EnblWrGroup1ofFmcregW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group2of_fmcreg(&mut self) -> EnblWrGroup2ofFmcregW<PricIo204Spec> {
        EnblWrGroup2ofFmcregW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group3of_fmcreg(&mut self) -> EnblWrGroup3ofFmcregW<PricIo204Spec> {
        EnblWrGroup3ofFmcregW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group4of_fmcreg(&mut self) -> EnblWrGroup4ofFmcregW<PricIo204Spec> {
        EnblWrGroup4ofFmcregW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of FMC Register"]
    #[inline(always)]
    pub fn enbl_wr_group5of_fmcreg(&mut self) -> EnblWrGroup5ofFmcregW<PricIo204Spec> {
        EnblWrGroup5ofFmcregW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1204PRIC1_204\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1204pric12040500(
        &mut self,
    ) -> EnblRstToleranceOfPric1204pric12040500W<PricIo204Spec> {
        EnblRstToleranceOfPric1204pric12040500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1204PRIC1_204\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1204pric12040600(
        &mut self,
    ) -> EnblWrProtOfPric1204pric12040600W<PricIo204Spec> {
        EnblWrProtOfPric1204pric12040600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi0reg(&mut self) -> EnblWrGroup0ofSpi0regW<PricIo204Spec> {
        EnblWrGroup0ofSpi0regW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi0reg(&mut self) -> EnblWrGroup1ofSpi0regW<PricIo204Spec> {
        EnblWrGroup1ofSpi0regW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi0reg(&mut self) -> EnblWrGroup2ofSpi0regW<PricIo204Spec> {
        EnblWrGroup2ofSpi0regW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi0reg(&mut self) -> EnblWrGroup3ofSpi0regW<PricIo204Spec> {
        EnblWrGroup3ofSpi0regW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi0reg(&mut self) -> EnblWrGroup4ofSpi0regW<PricIo204Spec> {
        EnblWrGroup4ofSpi0regW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi0reg(&mut self) -> EnblWrGroup5ofSpi0regW<PricIo204Spec> {
        EnblWrGroup5ofSpi0regW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1204PRIC1_204\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1204pric12041308(
        &mut self,
    ) -> EnblRstToleranceOfPric1204pric12041308W<PricIo204Spec> {
        EnblRstToleranceOfPric1204pric12041308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1204PRIC1_204\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1204pric12041408(
        &mut self,
    ) -> EnblWrProtOfPric1204pric12041408W<PricIo204Spec> {
        EnblWrProtOfPric1204pric12041408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group0of_spi1reg(&mut self) -> EnblWrGroup0ofSpi1regW<PricIo204Spec> {
        EnblWrGroup0ofSpi1regW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group1of_spi1reg(&mut self) -> EnblWrGroup1ofSpi1regW<PricIo204Spec> {
        EnblWrGroup1ofSpi1regW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group2of_spi1reg(&mut self) -> EnblWrGroup2ofSpi1regW<PricIo204Spec> {
        EnblWrGroup2ofSpi1regW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group3of_spi1reg(&mut self) -> EnblWrGroup3ofSpi1regW<PricIo204Spec> {
        EnblWrGroup3ofSpi1regW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group4of_spi1reg(&mut self) -> EnblWrGroup4ofSpi1regW<PricIo204Spec> {
        EnblWrGroup4ofSpi1regW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_wr_group5of_spi1reg(&mut self) -> EnblWrGroup5ofSpi1regW<PricIo204Spec> {
        EnblWrGroup5ofSpi1regW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1204PRIC1_204\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1204pric12042116(
        &mut self,
    ) -> EnblRstToleranceOfPric1204pric12042116W<PricIo204Spec> {
        EnblRstToleranceOfPric1204pric12042116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1204PRIC1_204\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1204pric12042216(
        &mut self,
    ) -> EnblWrProtOfPric1204pric12042216W<PricIo204Spec> {
        EnblWrProtOfPric1204pric12042216W::new(self, 23)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo204Spec> {
        Reserved7W::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo204Spec> {
        Reserved6W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo204Spec> {
        Reserved5W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo204Spec> {
        Reserved4W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo204Spec> {
        Reserved3W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo204Spec> {
        Reserved2W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo204Spec> {
        Reserved1W::new(self, 30)
    }
}
#[doc = "Slave Write Group Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io204::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io204::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo204Spec;
impl crate::RegisterSpec for PricIo204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io204::R`](R) reader structure"]
impl crate::Readable for PricIo204Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io204::W`](W) writer structure"]
impl crate::Writable for PricIo204Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO204 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo204Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
