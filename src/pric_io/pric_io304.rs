#[doc = "Register `PRIC_IO304` reader"]
pub type R = crate::R<PricIo304Spec>;
#[doc = "Register `PRIC_IO304` writer"]
pub type W = crate::W<PricIo304Spec>;
#[doc = "Field `EnblReadGroup0OfFMCReg` reader - Enable Read Group #0 of FMC Register"]
pub type EnblReadGroup0ofFmcregR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfFMCReg` writer - Enable Read Group #0 of FMC Register"]
pub type EnblReadGroup0ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfFMCReg` reader - Enable Read Group #1 of FMC Register"]
pub type EnblReadGroup1ofFmcregR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfFMCReg` writer - Enable Read Group #1 of FMC Register"]
pub type EnblReadGroup1ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfFMCReg` reader - Enable Read Group #2 of FMC Register"]
pub type EnblReadGroup2ofFmcregR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfFMCReg` writer - Enable Read Group #2 of FMC Register"]
pub type EnblReadGroup2ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfFMCReg` reader - Enable Read Group #3 of FMC Register"]
pub type EnblReadGroup3ofFmcregR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfFMCReg` writer - Enable Read Group #3 of FMC Register"]
pub type EnblReadGroup3ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfFMCReg` reader - Enable Read Group #4 of FMC Register"]
pub type EnblReadGroup4ofFmcregR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfFMCReg` writer - Enable Read Group #4 of FMC Register"]
pub type EnblReadGroup4ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfFMCReg` reader - Enable Read Group #5 of FMC Register"]
pub type EnblReadGroup5ofFmcregR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfFMCReg` writer - Enable Read Group #5 of FMC Register"]
pub type EnblReadGroup5ofFmcregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1304PRIC1_304\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1304pric13040500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1304pric13040500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1304pric13040500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1304PRIC13040500` reader - Enable Reset Tolerance of PRIC1304PRIC1_304\\[05:00\\]"]
pub type EnblRstToleranceOfPric1304pric13040500R =
    crate::BitReader<EnblRstToleranceOfPric1304pric13040500>;
impl EnblRstToleranceOfPric1304pric13040500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1304pric13040500 {
        match self.bits {
            false => EnblRstToleranceOfPric1304pric13040500::ResetBySrst,
            true => EnblRstToleranceOfPric1304pric13040500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1304pric13040500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1304pric13040500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1304PRIC13040500` writer - Enable Reset Tolerance of PRIC1304PRIC1_304\\[05:00\\]"]
pub type EnblRstToleranceOfPric1304pric13040500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1304pric13040500>;
impl<'a, REG> EnblRstToleranceOfPric1304pric13040500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1304pric13040500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1304pric13040500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1304PRIC13040600` reader - Enable Write Protection of PRIC1304PRIC1_304\\[06:00\\]"]
pub type EnblWrProtOfPric1304pric13040600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1304PRIC13040600` writer - Enable Write Protection of PRIC1304PRIC1_304\\[06:00\\]"]
pub type EnblWrProtOfPric1304pric13040600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSPI0Reg` reader - Enable Read Group #0 of SPI0 Register"]
pub type EnblReadGroup0ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSPI0Reg` writer - Enable Read Group #0 of SPI0 Register"]
pub type EnblReadGroup0ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSPI0Reg` reader - Enable Read Group #1 of SPI0 Register"]
pub type EnblReadGroup1ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSPI0Reg` writer - Enable Read Group #1 of SPI0 Register"]
pub type EnblReadGroup1ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSPI0Reg` reader - Enable Read Group #2 of SPI0 Register"]
pub type EnblReadGroup2ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSPI0Reg` writer - Enable Read Group #2 of SPI0 Register"]
pub type EnblReadGroup2ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSPI0Reg` reader - Enable Read Group #3 of SPI0 Register"]
pub type EnblReadGroup3ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSPI0Reg` writer - Enable Read Group #3 of SPI0 Register"]
pub type EnblReadGroup3ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSPI0Reg` reader - Enable Read Group #4 of SPI0 Register"]
pub type EnblReadGroup4ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSPI0Reg` writer - Enable Read Group #4 of SPI0 Register"]
pub type EnblReadGroup4ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSPI0Reg` reader - Enable Read Group #5 of SPI0 Register"]
pub type EnblReadGroup5ofSpi0regR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSPI0Reg` writer - Enable Read Group #5 of SPI0 Register"]
pub type EnblReadGroup5ofSpi0regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1304PRIC1_304\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1304pric13041308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1304pric13041308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1304pric13041308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1304PRIC13041308` reader - Enable Reset Tolerance of PRIC1304PRIC1_304\\[13:08\\]"]
pub type EnblRstToleranceOfPric1304pric13041308R =
    crate::BitReader<EnblRstToleranceOfPric1304pric13041308>;
impl EnblRstToleranceOfPric1304pric13041308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1304pric13041308 {
        match self.bits {
            false => EnblRstToleranceOfPric1304pric13041308::ResetBySrst,
            true => EnblRstToleranceOfPric1304pric13041308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1304pric13041308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1304pric13041308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1304PRIC13041308` writer - Enable Reset Tolerance of PRIC1304PRIC1_304\\[13:08\\]"]
pub type EnblRstToleranceOfPric1304pric13041308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1304pric13041308>;
impl<'a, REG> EnblRstToleranceOfPric1304pric13041308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1304pric13041308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1304pric13041308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1304PRIC13041408` reader - Enable Write Protection of PRIC1304PRIC1_304\\[14:08\\]"]
pub type EnblWrProtOfPric1304pric13041408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1304PRIC13041408` writer - Enable Write Protection of PRIC1304PRIC1_304\\[14:08\\]"]
pub type EnblWrProtOfPric1304pric13041408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfSPI1Reg` reader - Enable Read Group #0 of SPI1 Register"]
pub type EnblReadGroup0ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfSPI1Reg` writer - Enable Read Group #0 of SPI1 Register"]
pub type EnblReadGroup0ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfSPI1Reg` reader - Enable Read Group #1 of SPI1 Register"]
pub type EnblReadGroup1ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfSPI1Reg` writer - Enable Read Group #1 of SPI1 Register"]
pub type EnblReadGroup1ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfSPI1Reg` reader - Enable Read Group #2 of SPI1 Register"]
pub type EnblReadGroup2ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfSPI1Reg` writer - Enable Read Group #2 of SPI1 Register"]
pub type EnblReadGroup2ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfSPI1Reg` reader - Enable Read Group #3 of SPI1 Register"]
pub type EnblReadGroup3ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfSPI1Reg` writer - Enable Read Group #3 of SPI1 Register"]
pub type EnblReadGroup3ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfSPI1Reg` reader - Enable Read Group #4 of SPI1 Register"]
pub type EnblReadGroup4ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfSPI1Reg` writer - Enable Read Group #4 of SPI1 Register"]
pub type EnblReadGroup4ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfSPI1Reg` reader - Enable Read Group #5 of SPI1 Register"]
pub type EnblReadGroup5ofSpi1regR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfSPI1Reg` writer - Enable Read Group #5 of SPI1 Register"]
pub type EnblReadGroup5ofSpi1regW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1304PRIC1_304\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1304pric13042116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1304pric13042116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1304pric13042116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1304PRIC13042116` reader - Enable Reset Tolerance of PRIC1304PRIC1_304\\[21:16\\]"]
pub type EnblRstToleranceOfPric1304pric13042116R =
    crate::BitReader<EnblRstToleranceOfPric1304pric13042116>;
impl EnblRstToleranceOfPric1304pric13042116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1304pric13042116 {
        match self.bits {
            false => EnblRstToleranceOfPric1304pric13042116::ResetBySrst,
            true => EnblRstToleranceOfPric1304pric13042116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1304pric13042116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1304pric13042116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1304PRIC13042116` writer - Enable Reset Tolerance of PRIC1304PRIC1_304\\[21:16\\]"]
pub type EnblRstToleranceOfPric1304pric13042116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1304pric13042116>;
impl<'a, REG> EnblRstToleranceOfPric1304pric13042116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1304pric13042116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1304pric13042116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1304PRIC13042216` reader - Enable Write Protection of PRIC1304PRIC1_304\\[22:16\\]"]
pub type EnblWrProtOfPric1304pric13042216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1304PRIC13042216` writer - Enable Write Protection of PRIC1304PRIC1_304\\[22:16\\]"]
pub type EnblWrProtOfPric1304pric13042216W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - Enable Read Group #0 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group0of_fmcreg(&self) -> EnblReadGroup0ofFmcregR {
        EnblReadGroup0ofFmcregR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group1of_fmcreg(&self) -> EnblReadGroup1ofFmcregR {
        EnblReadGroup1ofFmcregR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group2of_fmcreg(&self) -> EnblReadGroup2ofFmcregR {
        EnblReadGroup2ofFmcregR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group3of_fmcreg(&self) -> EnblReadGroup3ofFmcregR {
        EnblReadGroup3ofFmcregR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group4of_fmcreg(&self) -> EnblReadGroup4ofFmcregR {
        EnblReadGroup4ofFmcregR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group5of_fmcreg(&self) -> EnblReadGroup5ofFmcregR {
        EnblReadGroup5ofFmcregR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1304PRIC1_304\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1304pric13040500(
        &self,
    ) -> EnblRstToleranceOfPric1304pric13040500R {
        EnblRstToleranceOfPric1304pric13040500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1304PRIC1_304\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1304pric13040600(&self) -> EnblWrProtOfPric1304pric13040600R {
        EnblWrProtOfPric1304pric13040600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi0reg(&self) -> EnblReadGroup0ofSpi0regR {
        EnblReadGroup0ofSpi0regR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi0reg(&self) -> EnblReadGroup1ofSpi0regR {
        EnblReadGroup1ofSpi0regR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi0reg(&self) -> EnblReadGroup2ofSpi0regR {
        EnblReadGroup2ofSpi0regR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi0reg(&self) -> EnblReadGroup3ofSpi0regR {
        EnblReadGroup3ofSpi0regR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi0reg(&self) -> EnblReadGroup4ofSpi0regR {
        EnblReadGroup4ofSpi0regR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi0reg(&self) -> EnblReadGroup5ofSpi0regR {
        EnblReadGroup5ofSpi0regR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1304PRIC1_304\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1304pric13041308(
        &self,
    ) -> EnblRstToleranceOfPric1304pric13041308R {
        EnblRstToleranceOfPric1304pric13041308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1304PRIC1_304\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1304pric13041408(&self) -> EnblWrProtOfPric1304pric13041408R {
        EnblWrProtOfPric1304pric13041408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi1reg(&self) -> EnblReadGroup0ofSpi1regR {
        EnblReadGroup0ofSpi1regR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi1reg(&self) -> EnblReadGroup1ofSpi1regR {
        EnblReadGroup1ofSpi1regR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi1reg(&self) -> EnblReadGroup2ofSpi1regR {
        EnblReadGroup2ofSpi1regR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi1reg(&self) -> EnblReadGroup3ofSpi1regR {
        EnblReadGroup3ofSpi1regR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi1reg(&self) -> EnblReadGroup4ofSpi1regR {
        EnblReadGroup4ofSpi1regR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi1reg(&self) -> EnblReadGroup5ofSpi1regR {
        EnblReadGroup5ofSpi1regR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1304PRIC1_304\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1304pric13042116(
        &self,
    ) -> EnblRstToleranceOfPric1304pric13042116R {
        EnblRstToleranceOfPric1304pric13042116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1304PRIC1_304\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1304pric13042216(&self) -> EnblWrProtOfPric1304pric13042216R {
        EnblWrProtOfPric1304pric13042216R::new(((self.bits >> 23) & 1) != 0)
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
    #[doc = "Bit 0 - Enable Read Group #0 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group0of_fmcreg(&mut self) -> EnblReadGroup0ofFmcregW<PricIo304Spec> {
        EnblReadGroup0ofFmcregW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group1of_fmcreg(&mut self) -> EnblReadGroup1ofFmcregW<PricIo304Spec> {
        EnblReadGroup1ofFmcregW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group2of_fmcreg(&mut self) -> EnblReadGroup2ofFmcregW<PricIo304Spec> {
        EnblReadGroup2ofFmcregW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group3of_fmcreg(&mut self) -> EnblReadGroup3ofFmcregW<PricIo304Spec> {
        EnblReadGroup3ofFmcregW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group4of_fmcreg(&mut self) -> EnblReadGroup4ofFmcregW<PricIo304Spec> {
        EnblReadGroup4ofFmcregW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of FMC Register"]
    #[inline(always)]
    pub fn enbl_read_group5of_fmcreg(&mut self) -> EnblReadGroup5ofFmcregW<PricIo304Spec> {
        EnblReadGroup5ofFmcregW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1304PRIC1_304\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1304pric13040500(
        &mut self,
    ) -> EnblRstToleranceOfPric1304pric13040500W<PricIo304Spec> {
        EnblRstToleranceOfPric1304pric13040500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1304PRIC1_304\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1304pric13040600(
        &mut self,
    ) -> EnblWrProtOfPric1304pric13040600W<PricIo304Spec> {
        EnblWrProtOfPric1304pric13040600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi0reg(&mut self) -> EnblReadGroup0ofSpi0regW<PricIo304Spec> {
        EnblReadGroup0ofSpi0regW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi0reg(&mut self) -> EnblReadGroup1ofSpi0regW<PricIo304Spec> {
        EnblReadGroup1ofSpi0regW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi0reg(&mut self) -> EnblReadGroup2ofSpi0regW<PricIo304Spec> {
        EnblReadGroup2ofSpi0regW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi0reg(&mut self) -> EnblReadGroup3ofSpi0regW<PricIo304Spec> {
        EnblReadGroup3ofSpi0regW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi0reg(&mut self) -> EnblReadGroup4ofSpi0regW<PricIo304Spec> {
        EnblReadGroup4ofSpi0regW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of SPI0 Register"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi0reg(&mut self) -> EnblReadGroup5ofSpi0regW<PricIo304Spec> {
        EnblReadGroup5ofSpi0regW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1304PRIC1_304\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1304pric13041308(
        &mut self,
    ) -> EnblRstToleranceOfPric1304pric13041308W<PricIo304Spec> {
        EnblRstToleranceOfPric1304pric13041308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1304PRIC1_304\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1304pric13041408(
        &mut self,
    ) -> EnblWrProtOfPric1304pric13041408W<PricIo304Spec> {
        EnblWrProtOfPric1304pric13041408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group0of_spi1reg(&mut self) -> EnblReadGroup0ofSpi1regW<PricIo304Spec> {
        EnblReadGroup0ofSpi1regW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group1of_spi1reg(&mut self) -> EnblReadGroup1ofSpi1regW<PricIo304Spec> {
        EnblReadGroup1ofSpi1regW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group2of_spi1reg(&mut self) -> EnblReadGroup2ofSpi1regW<PricIo304Spec> {
        EnblReadGroup2ofSpi1regW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group3of_spi1reg(&mut self) -> EnblReadGroup3ofSpi1regW<PricIo304Spec> {
        EnblReadGroup3ofSpi1regW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group4of_spi1reg(&mut self) -> EnblReadGroup4ofSpi1regW<PricIo304Spec> {
        EnblReadGroup4ofSpi1regW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of SPI1 Register"]
    #[inline(always)]
    pub fn enbl_read_group5of_spi1reg(&mut self) -> EnblReadGroup5ofSpi1regW<PricIo304Spec> {
        EnblReadGroup5ofSpi1regW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1304PRIC1_304\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1304pric13042116(
        &mut self,
    ) -> EnblRstToleranceOfPric1304pric13042116W<PricIo304Spec> {
        EnblRstToleranceOfPric1304pric13042116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1304PRIC1_304\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1304pric13042216(
        &mut self,
    ) -> EnblWrProtOfPric1304pric13042216W<PricIo304Spec> {
        EnblWrProtOfPric1304pric13042216W::new(self, 23)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo304Spec> {
        Reserved7W::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo304Spec> {
        Reserved6W::new(self, 25)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo304Spec> {
        Reserved5W::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo304Spec> {
        Reserved4W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo304Spec> {
        Reserved3W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo304Spec> {
        Reserved2W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo304Spec> {
        Reserved1W::new(self, 30)
    }
}
#[doc = "Slave Read Group Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io304::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io304::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo304Spec;
impl crate::RegisterSpec for PricIo304Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io304::R`](R) reader structure"]
impl crate::Readable for PricIo304Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io304::W`](W) writer structure"]
impl crate::Writable for PricIo304Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO304 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo304Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
