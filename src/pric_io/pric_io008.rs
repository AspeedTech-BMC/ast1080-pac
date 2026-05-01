#[doc = "Register `PRIC_IO008` reader"]
pub type R = crate::R<PricIo008Spec>;
#[doc = "Register `PRIC_IO008` writer"]
pub type W = crate::W<PricIo008Spec>;
#[doc = "Field `EnblWrGroup0OfESPIAccess` reader - Enable Write Group #0 of eSPI access"]
pub type EnblWrGroup0ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfESPIAccess` writer - Enable Write Group #0 of eSPI access"]
pub type EnblWrGroup0ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfESPIAccess` reader - Enable Write Group #1 of eSPI access"]
pub type EnblWrGroup1ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfESPIAccess` writer - Enable Write Group #1 of eSPI access"]
pub type EnblWrGroup1ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfESPIAccess` reader - Enable Write Group #2 of eSPI access"]
pub type EnblWrGroup2ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfESPIAccess` writer - Enable Write Group #2 of eSPI access"]
pub type EnblWrGroup2ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfESPIAccess` reader - Enable Write Group #3 of eSPI access"]
pub type EnblWrGroup3ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfESPIAccess` writer - Enable Write Group #3 of eSPI access"]
pub type EnblWrGroup3ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfESPIAccess` reader - Enable Write Group #4 of eSPI access"]
pub type EnblWrGroup4ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfESPIAccess` writer - Enable Write Group #4 of eSPI access"]
pub type EnblWrGroup4ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfESPIAccess` reader - Enable Write Group #5 of eSPI access"]
pub type EnblWrGroup5ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfESPIAccess` writer - Enable Write Group #5 of eSPI access"]
pub type EnblWrGroup5ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1008PRIC1_008\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1008pric10080500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1008pric10080500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1008pric10080500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1008PRIC10080500` reader - Enable Reset Tolerance of PRIC1008PRIC1_008\\[05:00\\]"]
pub type EnblRstToleranceOfPric1008pric10080500R =
    crate::BitReader<EnblRstToleranceOfPric1008pric10080500>;
impl EnblRstToleranceOfPric1008pric10080500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1008pric10080500 {
        match self.bits {
            false => EnblRstToleranceOfPric1008pric10080500::ResetBySrst,
            true => EnblRstToleranceOfPric1008pric10080500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1008pric10080500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1008pric10080500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1008PRIC10080500` writer - Enable Reset Tolerance of PRIC1008PRIC1_008\\[05:00\\]"]
pub type EnblRstToleranceOfPric1008pric10080500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1008pric10080500>;
impl<'a, REG> EnblRstToleranceOfPric1008pric10080500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1008pric10080500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1008pric10080500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1008PRIC10080600` reader - Enable Write Protection of PRIC1008PRIC1_008\\[06:00\\]"]
pub type EnblWrProtOfPric1008pric10080600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1008PRIC10080600` writer - Enable Write Protection of PRIC1008PRIC1_008\\[06:00\\]"]
pub type EnblWrProtOfPric1008pric10080600W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `EnblWrGroup0OfPostCodeDMAAccess` reader - Enable Write Group #0 of PostCode DMA access"]
pub type EnblWrGroup0ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfPostCodeDMAAccess` writer - Enable Write Group #0 of PostCode DMA access"]
pub type EnblWrGroup0ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfPostCodeDMAAccess` reader - Enable Write Group #1 of PostCode DMA access"]
pub type EnblWrGroup1ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfPostCodeDMAAccess` writer - Enable Write Group #1 of PostCode DMA access"]
pub type EnblWrGroup1ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfPostCodeDMAAccess` reader - Enable Write Group #2 of PostCode DMA access"]
pub type EnblWrGroup2ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfPostCodeDMAAccess` writer - Enable Write Group #2 of PostCode DMA access"]
pub type EnblWrGroup2ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfPostCodeDMAAccess` reader - Enable Write Group #3 of PostCode DMA access"]
pub type EnblWrGroup3ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfPostCodeDMAAccess` writer - Enable Write Group #3 of PostCode DMA access"]
pub type EnblWrGroup3ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfPostCodeDMAAccess` reader - Enable Write Group #4 of PostCode DMA access"]
pub type EnblWrGroup4ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfPostCodeDMAAccess` writer - Enable Write Group #4 of PostCode DMA access"]
pub type EnblWrGroup4ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfPostCodeDMAAccess` reader - Enable Write Group #5 of PostCode DMA access"]
pub type EnblWrGroup5ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfPostCodeDMAAccess` writer - Enable Write Group #5 of PostCode DMA access"]
pub type EnblWrGroup5ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1008PRIC1_008\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1008pric10082116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1008pric10082116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1008pric10082116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1008PRIC10082116` reader - Enable Reset Tolerance of PRIC1008PRIC1_008\\[21:16\\]"]
pub type EnblRstToleranceOfPric1008pric10082116R =
    crate::BitReader<EnblRstToleranceOfPric1008pric10082116>;
impl EnblRstToleranceOfPric1008pric10082116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1008pric10082116 {
        match self.bits {
            false => EnblRstToleranceOfPric1008pric10082116::ResetBySrst,
            true => EnblRstToleranceOfPric1008pric10082116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1008pric10082116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1008pric10082116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1008PRIC10082116` writer - Enable Reset Tolerance of PRIC1008PRIC1_008\\[21:16\\]"]
pub type EnblRstToleranceOfPric1008pric10082116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1008pric10082116>;
impl<'a, REG> EnblRstToleranceOfPric1008pric10082116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1008pric10082116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1008pric10082116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1008PRIC10082216` reader - Enable Write Protection of PRIC1008PRIC1_008\\[22:16\\]"]
pub type EnblWrProtOfPric1008pric10082216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1008PRIC10082216` writer - Enable Write Protection of PRIC1008PRIC1_008\\[22:16\\]"]
pub type EnblWrProtOfPric1008pric10082216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUartDbgAccess` reader - Enable Write Group #0 of UartDbg access"]
pub type EnblWrGroup0ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUartDbgAccess` writer - Enable Write Group #0 of UartDbg access"]
pub type EnblWrGroup0ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUartDbgAccess` reader - Enable Write Group #1 of UartDbg access"]
pub type EnblWrGroup1ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUartDbgAccess` writer - Enable Write Group #1 of UartDbg access"]
pub type EnblWrGroup1ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUartDbgAccess` reader - Enable Write Group #2 of UartDbg access"]
pub type EnblWrGroup2ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUartDbgAccess` writer - Enable Write Group #2 of UartDbg access"]
pub type EnblWrGroup2ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUartDbgAccess` reader - Enable Write Group #3 of UartDbg access"]
pub type EnblWrGroup3ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUartDbgAccess` writer - Enable Write Group #3 of UartDbg access"]
pub type EnblWrGroup3ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUartDbgAccess` reader - Enable Write Group #4 of UartDbg access"]
pub type EnblWrGroup4ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUartDbgAccess` writer - Enable Write Group #4 of UartDbg access"]
pub type EnblWrGroup4ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUartDbgAccess` reader - Enable Write Group #5 of UartDbg access"]
pub type EnblWrGroup5ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUartDbgAccess` writer - Enable Write Group #5 of UartDbg access"]
pub type EnblWrGroup5ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1008PRIC1_008\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1008pric10082924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1008pric10082924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1008pric10082924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1008PRIC10082924` reader - Enable Reset Tolerance of PRIC1008PRIC1_008\\[29:24\\]"]
pub type EnblRstToleranceOfPric1008pric10082924R =
    crate::BitReader<EnblRstToleranceOfPric1008pric10082924>;
impl EnblRstToleranceOfPric1008pric10082924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1008pric10082924 {
        match self.bits {
            false => EnblRstToleranceOfPric1008pric10082924::ResetBySrst,
            true => EnblRstToleranceOfPric1008pric10082924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1008pric10082924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1008pric10082924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1008PRIC10082924` writer - Enable Reset Tolerance of PRIC1008PRIC1_008\\[29:24\\]"]
pub type EnblRstToleranceOfPric1008pric10082924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1008pric10082924>;
impl<'a, REG> EnblRstToleranceOfPric1008pric10082924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1008pric10082924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1008pric10082924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1008PRIC10083024` reader - Enable Write Protection of PRIC1008PRIC1_008\\[30:24\\]"]
pub type EnblWrProtOfPric1008pric10083024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1008PRIC10083024` writer - Enable Write Protection of PRIC1008PRIC1_008\\[30:24\\]"]
pub type EnblWrProtOfPric1008pric10083024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espiaccess(&self) -> EnblWrGroup0ofEspiaccessR {
        EnblWrGroup0ofEspiaccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espiaccess(&self) -> EnblWrGroup1ofEspiaccessR {
        EnblWrGroup1ofEspiaccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espiaccess(&self) -> EnblWrGroup2ofEspiaccessR {
        EnblWrGroup2ofEspiaccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espiaccess(&self) -> EnblWrGroup3ofEspiaccessR {
        EnblWrGroup3ofEspiaccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espiaccess(&self) -> EnblWrGroup4ofEspiaccessR {
        EnblWrGroup4ofEspiaccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espiaccess(&self) -> EnblWrGroup5ofEspiaccessR {
        EnblWrGroup5ofEspiaccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1008PRIC1_008\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1008pric10080500(
        &self,
    ) -> EnblRstToleranceOfPric1008pric10080500R {
        EnblRstToleranceOfPric1008pric10080500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1008PRIC1_008\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1008pric10080600(&self) -> EnblWrProtOfPric1008pric10080600R {
        EnblWrProtOfPric1008pric10080600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_post_code_dmaaccess(&self) -> EnblWrGroup0ofPostCodeDmaaccessR {
        EnblWrGroup0ofPostCodeDmaaccessR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_post_code_dmaaccess(&self) -> EnblWrGroup1ofPostCodeDmaaccessR {
        EnblWrGroup1ofPostCodeDmaaccessR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_post_code_dmaaccess(&self) -> EnblWrGroup2ofPostCodeDmaaccessR {
        EnblWrGroup2ofPostCodeDmaaccessR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_post_code_dmaaccess(&self) -> EnblWrGroup3ofPostCodeDmaaccessR {
        EnblWrGroup3ofPostCodeDmaaccessR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_post_code_dmaaccess(&self) -> EnblWrGroup4ofPostCodeDmaaccessR {
        EnblWrGroup4ofPostCodeDmaaccessR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_post_code_dmaaccess(&self) -> EnblWrGroup5ofPostCodeDmaaccessR {
        EnblWrGroup5ofPostCodeDmaaccessR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1008PRIC1_008\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1008pric10082116(
        &self,
    ) -> EnblRstToleranceOfPric1008pric10082116R {
        EnblRstToleranceOfPric1008pric10082116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1008PRIC1_008\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1008pric10082216(&self) -> EnblWrProtOfPric1008pric10082216R {
        EnblWrProtOfPric1008pric10082216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart_dbg_access(&self) -> EnblWrGroup0ofUartDbgAccessR {
        EnblWrGroup0ofUartDbgAccessR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart_dbg_access(&self) -> EnblWrGroup1ofUartDbgAccessR {
        EnblWrGroup1ofUartDbgAccessR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart_dbg_access(&self) -> EnblWrGroup2ofUartDbgAccessR {
        EnblWrGroup2ofUartDbgAccessR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart_dbg_access(&self) -> EnblWrGroup3ofUartDbgAccessR {
        EnblWrGroup3ofUartDbgAccessR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart_dbg_access(&self) -> EnblWrGroup4ofUartDbgAccessR {
        EnblWrGroup4ofUartDbgAccessR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart_dbg_access(&self) -> EnblWrGroup5ofUartDbgAccessR {
        EnblWrGroup5ofUartDbgAccessR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1008PRIC1_008\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1008pric10082924(
        &self,
    ) -> EnblRstToleranceOfPric1008pric10082924R {
        EnblRstToleranceOfPric1008pric10082924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1008PRIC1_008\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1008pric10083024(&self) -> EnblWrProtOfPric1008pric10083024R {
        EnblWrProtOfPric1008pric10083024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_espiaccess(&mut self) -> EnblWrGroup0ofEspiaccessW<PricIo008Spec> {
        EnblWrGroup0ofEspiaccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_espiaccess(&mut self) -> EnblWrGroup1ofEspiaccessW<PricIo008Spec> {
        EnblWrGroup1ofEspiaccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_espiaccess(&mut self) -> EnblWrGroup2ofEspiaccessW<PricIo008Spec> {
        EnblWrGroup2ofEspiaccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_espiaccess(&mut self) -> EnblWrGroup3ofEspiaccessW<PricIo008Spec> {
        EnblWrGroup3ofEspiaccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_espiaccess(&mut self) -> EnblWrGroup4ofEspiaccessW<PricIo008Spec> {
        EnblWrGroup4ofEspiaccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of eSPI access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_espiaccess(&mut self) -> EnblWrGroup5ofEspiaccessW<PricIo008Spec> {
        EnblWrGroup5ofEspiaccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1008PRIC1_008\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1008pric10080500(
        &mut self,
    ) -> EnblRstToleranceOfPric1008pric10080500W<PricIo008Spec> {
        EnblRstToleranceOfPric1008pric10080500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1008PRIC1_008\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1008pric10080600(
        &mut self,
    ) -> EnblWrProtOfPric1008pric10080600W<PricIo008Spec> {
        EnblWrProtOfPric1008pric10080600W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo008Spec> {
        Reserved7W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo008Spec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo008Spec> {
        Reserved5W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo008Spec> {
        Reserved4W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo008Spec> {
        Reserved3W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo008Spec> {
        Reserved2W::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo008Spec> {
        Reserved1W::new(self, 14)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_post_code_dmaaccess(
        &mut self,
    ) -> EnblWrGroup0ofPostCodeDmaaccessW<PricIo008Spec> {
        EnblWrGroup0ofPostCodeDmaaccessW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_post_code_dmaaccess(
        &mut self,
    ) -> EnblWrGroup1ofPostCodeDmaaccessW<PricIo008Spec> {
        EnblWrGroup1ofPostCodeDmaaccessW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_post_code_dmaaccess(
        &mut self,
    ) -> EnblWrGroup2ofPostCodeDmaaccessW<PricIo008Spec> {
        EnblWrGroup2ofPostCodeDmaaccessW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_post_code_dmaaccess(
        &mut self,
    ) -> EnblWrGroup3ofPostCodeDmaaccessW<PricIo008Spec> {
        EnblWrGroup3ofPostCodeDmaaccessW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_post_code_dmaaccess(
        &mut self,
    ) -> EnblWrGroup4ofPostCodeDmaaccessW<PricIo008Spec> {
        EnblWrGroup4ofPostCodeDmaaccessW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_post_code_dmaaccess(
        &mut self,
    ) -> EnblWrGroup5ofPostCodeDmaaccessW<PricIo008Spec> {
        EnblWrGroup5ofPostCodeDmaaccessW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1008PRIC1_008\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1008pric10082116(
        &mut self,
    ) -> EnblRstToleranceOfPric1008pric10082116W<PricIo008Spec> {
        EnblRstToleranceOfPric1008pric10082116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1008PRIC1_008\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1008pric10082216(
        &mut self,
    ) -> EnblWrProtOfPric1008pric10082216W<PricIo008Spec> {
        EnblWrProtOfPric1008pric10082216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uart_dbg_access(
        &mut self,
    ) -> EnblWrGroup0ofUartDbgAccessW<PricIo008Spec> {
        EnblWrGroup0ofUartDbgAccessW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uart_dbg_access(
        &mut self,
    ) -> EnblWrGroup1ofUartDbgAccessW<PricIo008Spec> {
        EnblWrGroup1ofUartDbgAccessW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uart_dbg_access(
        &mut self,
    ) -> EnblWrGroup2ofUartDbgAccessW<PricIo008Spec> {
        EnblWrGroup2ofUartDbgAccessW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uart_dbg_access(
        &mut self,
    ) -> EnblWrGroup3ofUartDbgAccessW<PricIo008Spec> {
        EnblWrGroup3ofUartDbgAccessW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uart_dbg_access(
        &mut self,
    ) -> EnblWrGroup4ofUartDbgAccessW<PricIo008Spec> {
        EnblWrGroup4ofUartDbgAccessW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uart_dbg_access(
        &mut self,
    ) -> EnblWrGroup5ofUartDbgAccessW<PricIo008Spec> {
        EnblWrGroup5ofUartDbgAccessW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1008PRIC1_008\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1008pric10082924(
        &mut self,
    ) -> EnblRstToleranceOfPric1008pric10082924W<PricIo008Spec> {
        EnblRstToleranceOfPric1008pric10082924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1008PRIC1_008\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1008pric10083024(
        &mut self,
    ) -> EnblWrProtOfPric1008pric10083024W<PricIo008Spec> {
        EnblWrProtOfPric1008pric10083024W::new(self, 31)
    }
}
#[doc = "Master Write Group Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo008Spec;
impl crate::RegisterSpec for PricIo008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io008::R`](R) reader structure"]
impl crate::Readable for PricIo008Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io008::W`](W) writer structure"]
impl crate::Writable for PricIo008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO008 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo008Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
