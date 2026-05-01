#[doc = "Register `PRIC_IO108` reader"]
pub type R = crate::R<PricIo108Spec>;
#[doc = "Register `PRIC_IO108` writer"]
pub type W = crate::W<PricIo108Spec>;
#[doc = "Field `EnblReadGroup0OfESPIAccess` reader - Enable Read Group #0 of eSPI access"]
pub type EnblReadGroup0ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfESPIAccess` writer - Enable Read Group #0 of eSPI access"]
pub type EnblReadGroup0ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfESPIAccess` reader - Enable Read Group #1 of eSPI access"]
pub type EnblReadGroup1ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfESPIAccess` writer - Enable Read Group #1 of eSPI access"]
pub type EnblReadGroup1ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfESPIAccess` reader - Enable Read Group #2 of eSPI access"]
pub type EnblReadGroup2ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfESPIAccess` writer - Enable Read Group #2 of eSPI access"]
pub type EnblReadGroup2ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfESPIAccess` reader - Enable Read Group #3 of eSPI access"]
pub type EnblReadGroup3ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfESPIAccess` writer - Enable Read Group #3 of eSPI access"]
pub type EnblReadGroup3ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfESPIAccess` reader - Enable Read Group #4 of eSPI access"]
pub type EnblReadGroup4ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfESPIAccess` writer - Enable Read Group #4 of eSPI access"]
pub type EnblReadGroup4ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfESPIAccess` reader - Enable Read Group #5 of eSPI access"]
pub type EnblReadGroup5ofEspiaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfESPIAccess` writer - Enable Read Group #5 of eSPI access"]
pub type EnblReadGroup5ofEspiaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1108PRIC1_108\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1108pric11080500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1108pric11080500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1108pric11080500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1108PRIC11080500` reader - Enable Reset Tolerance of PRIC1108PRIC1_108\\[05:00\\]"]
pub type EnblRstToleranceOfPric1108pric11080500R =
    crate::BitReader<EnblRstToleranceOfPric1108pric11080500>;
impl EnblRstToleranceOfPric1108pric11080500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1108pric11080500 {
        match self.bits {
            false => EnblRstToleranceOfPric1108pric11080500::ResetBySrst,
            true => EnblRstToleranceOfPric1108pric11080500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1108pric11080500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1108pric11080500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1108PRIC11080500` writer - Enable Reset Tolerance of PRIC1108PRIC1_108\\[05:00\\]"]
pub type EnblRstToleranceOfPric1108pric11080500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1108pric11080500>;
impl<'a, REG> EnblRstToleranceOfPric1108pric11080500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1108pric11080500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1108pric11080500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1108PRIC11080600` reader - Enable Write Protection of PRIC1108PRIC1_108\\[06:00\\]"]
pub type EnblWrProtOfPric1108pric11080600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1108PRIC11080600` writer - Enable Write Protection of PRIC1108PRIC1_108\\[06:00\\]"]
pub type EnblWrProtOfPric1108pric11080600W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `EnblReadGroup0OfPostCodeDMAAccess` reader - Enable Read Group #0 of PostCode DMA access"]
pub type EnblReadGroup0ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfPostCodeDMAAccess` writer - Enable Read Group #0 of PostCode DMA access"]
pub type EnblReadGroup0ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfPostCodeDMAAccess` reader - Enable Read Group #1 of PostCode DMA access"]
pub type EnblReadGroup1ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfPostCodeDMAAccess` writer - Enable Read Group #1 of PostCode DMA access"]
pub type EnblReadGroup1ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfPostCodeDMAAccess` reader - Enable Read Group #2 of PostCode DMA access"]
pub type EnblReadGroup2ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfPostCodeDMAAccess` writer - Enable Read Group #2 of PostCode DMA access"]
pub type EnblReadGroup2ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfPostCodeDMAAccess` reader - Enable Read Group #3 of PostCode DMA access"]
pub type EnblReadGroup3ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfPostCodeDMAAccess` writer - Enable Read Group #3 of PostCode DMA access"]
pub type EnblReadGroup3ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfPostCodeDMAAccess` reader - Enable Read Group #4 of PostCode DMA access"]
pub type EnblReadGroup4ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfPostCodeDMAAccess` writer - Enable Read Group #4 of PostCode DMA access"]
pub type EnblReadGroup4ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfPostCodeDMAAccess` reader - Enable Read Group #5 of PostCode DMA access"]
pub type EnblReadGroup5ofPostCodeDmaaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfPostCodeDMAAccess` writer - Enable Read Group #5 of PostCode DMA access"]
pub type EnblReadGroup5ofPostCodeDmaaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1108PRIC1_108\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1108pric11082116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1108pric11082116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1108pric11082116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1108PRIC11082116` reader - Enable Reset Tolerance of PRIC1108PRIC1_108\\[21:16\\]"]
pub type EnblRstToleranceOfPric1108pric11082116R =
    crate::BitReader<EnblRstToleranceOfPric1108pric11082116>;
impl EnblRstToleranceOfPric1108pric11082116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1108pric11082116 {
        match self.bits {
            false => EnblRstToleranceOfPric1108pric11082116::ResetBySrst,
            true => EnblRstToleranceOfPric1108pric11082116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1108pric11082116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1108pric11082116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1108PRIC11082116` writer - Enable Reset Tolerance of PRIC1108PRIC1_108\\[21:16\\]"]
pub type EnblRstToleranceOfPric1108pric11082116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1108pric11082116>;
impl<'a, REG> EnblRstToleranceOfPric1108pric11082116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1108pric11082116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1108pric11082116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1108PRIC11082216` reader - Enable Write Protection of PRIC1108PRIC1_108\\[22:16\\]"]
pub type EnblWrProtOfPric1108pric11082216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1108PRIC11082216` writer - Enable Write Protection of PRIC1108PRIC1_108\\[22:16\\]"]
pub type EnblWrProtOfPric1108pric11082216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUartDbgAccess` reader - Enable Read Group #0 of UartDbg access"]
pub type EnblReadGroup0ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUartDbgAccess` writer - Enable Read Group #0 of UartDbg access"]
pub type EnblReadGroup0ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUartDbgAccess` reader - Enable Read Group #1 of UartDbg access"]
pub type EnblReadGroup1ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUartDbgAccess` writer - Enable Read Group #1 of UartDbg access"]
pub type EnblReadGroup1ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUartDbgAccess` reader - Enable Read Group #2 of UartDbg access"]
pub type EnblReadGroup2ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUartDbgAccess` writer - Enable Read Group #2 of UartDbg access"]
pub type EnblReadGroup2ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUartDbgAccess` reader - Enable Read Group #3 of UartDbg access"]
pub type EnblReadGroup3ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUartDbgAccess` writer - Enable Read Group #3 of UartDbg access"]
pub type EnblReadGroup3ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUartDbgAccess` reader - Enable Read Group #4 of UartDbg access"]
pub type EnblReadGroup4ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUartDbgAccess` writer - Enable Read Group #4 of UartDbg access"]
pub type EnblReadGroup4ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUartDbgAccess` reader - Enable Read Group #5 of UartDbg access"]
pub type EnblReadGroup5ofUartDbgAccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUartDbgAccess` writer - Enable Read Group #5 of UartDbg access"]
pub type EnblReadGroup5ofUartDbgAccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1108PRIC1_108\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1108pric11082924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1108pric11082924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1108pric11082924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1108PRIC11082924` reader - Enable Reset Tolerance of PRIC1108PRIC1_108\\[29:24\\]"]
pub type EnblRstToleranceOfPric1108pric11082924R =
    crate::BitReader<EnblRstToleranceOfPric1108pric11082924>;
impl EnblRstToleranceOfPric1108pric11082924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1108pric11082924 {
        match self.bits {
            false => EnblRstToleranceOfPric1108pric11082924::ResetBySrst,
            true => EnblRstToleranceOfPric1108pric11082924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1108pric11082924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1108pric11082924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1108PRIC11082924` writer - Enable Reset Tolerance of PRIC1108PRIC1_108\\[29:24\\]"]
pub type EnblRstToleranceOfPric1108pric11082924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1108pric11082924>;
impl<'a, REG> EnblRstToleranceOfPric1108pric11082924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1108pric11082924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1108pric11082924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1108PRIC11083024` reader - Enable Write Protection of PRIC1108PRIC1_108\\[30:24\\]"]
pub type EnblWrProtOfPric1108pric11083024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1108PRIC11083024` writer - Enable Write Protection of PRIC1108PRIC1_108\\[30:24\\]"]
pub type EnblWrProtOfPric1108pric11083024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group0of_espiaccess(&self) -> EnblReadGroup0ofEspiaccessR {
        EnblReadGroup0ofEspiaccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group1of_espiaccess(&self) -> EnblReadGroup1ofEspiaccessR {
        EnblReadGroup1ofEspiaccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group2of_espiaccess(&self) -> EnblReadGroup2ofEspiaccessR {
        EnblReadGroup2ofEspiaccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group3of_espiaccess(&self) -> EnblReadGroup3ofEspiaccessR {
        EnblReadGroup3ofEspiaccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group4of_espiaccess(&self) -> EnblReadGroup4ofEspiaccessR {
        EnblReadGroup4ofEspiaccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group5of_espiaccess(&self) -> EnblReadGroup5ofEspiaccessR {
        EnblReadGroup5ofEspiaccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1108PRIC1_108\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1108pric11080500(
        &self,
    ) -> EnblRstToleranceOfPric1108pric11080500R {
        EnblRstToleranceOfPric1108pric11080500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1108PRIC1_108\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1108pric11080600(&self) -> EnblWrProtOfPric1108pric11080600R {
        EnblWrProtOfPric1108pric11080600R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 16 - Enable Read Group #0 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group0of_post_code_dmaaccess(&self) -> EnblReadGroup0ofPostCodeDmaaccessR {
        EnblReadGroup0ofPostCodeDmaaccessR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group1of_post_code_dmaaccess(&self) -> EnblReadGroup1ofPostCodeDmaaccessR {
        EnblReadGroup1ofPostCodeDmaaccessR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group2of_post_code_dmaaccess(&self) -> EnblReadGroup2ofPostCodeDmaaccessR {
        EnblReadGroup2ofPostCodeDmaaccessR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group3of_post_code_dmaaccess(&self) -> EnblReadGroup3ofPostCodeDmaaccessR {
        EnblReadGroup3ofPostCodeDmaaccessR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group4of_post_code_dmaaccess(&self) -> EnblReadGroup4ofPostCodeDmaaccessR {
        EnblReadGroup4ofPostCodeDmaaccessR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group5of_post_code_dmaaccess(&self) -> EnblReadGroup5ofPostCodeDmaaccessR {
        EnblReadGroup5ofPostCodeDmaaccessR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1108PRIC1_108\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1108pric11082116(
        &self,
    ) -> EnblRstToleranceOfPric1108pric11082116R {
        EnblRstToleranceOfPric1108pric11082116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1108PRIC1_108\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1108pric11082216(&self) -> EnblWrProtOfPric1108pric11082216R {
        EnblWrProtOfPric1108pric11082216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart_dbg_access(&self) -> EnblReadGroup0ofUartDbgAccessR {
        EnblReadGroup0ofUartDbgAccessR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart_dbg_access(&self) -> EnblReadGroup1ofUartDbgAccessR {
        EnblReadGroup1ofUartDbgAccessR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart_dbg_access(&self) -> EnblReadGroup2ofUartDbgAccessR {
        EnblReadGroup2ofUartDbgAccessR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart_dbg_access(&self) -> EnblReadGroup3ofUartDbgAccessR {
        EnblReadGroup3ofUartDbgAccessR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart_dbg_access(&self) -> EnblReadGroup4ofUartDbgAccessR {
        EnblReadGroup4ofUartDbgAccessR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart_dbg_access(&self) -> EnblReadGroup5ofUartDbgAccessR {
        EnblReadGroup5ofUartDbgAccessR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1108PRIC1_108\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1108pric11082924(
        &self,
    ) -> EnblRstToleranceOfPric1108pric11082924R {
        EnblRstToleranceOfPric1108pric11082924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1108PRIC1_108\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1108pric11083024(&self) -> EnblWrProtOfPric1108pric11083024R {
        EnblWrProtOfPric1108pric11083024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group0of_espiaccess(&mut self) -> EnblReadGroup0ofEspiaccessW<PricIo108Spec> {
        EnblReadGroup0ofEspiaccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group1of_espiaccess(&mut self) -> EnblReadGroup1ofEspiaccessW<PricIo108Spec> {
        EnblReadGroup1ofEspiaccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group2of_espiaccess(&mut self) -> EnblReadGroup2ofEspiaccessW<PricIo108Spec> {
        EnblReadGroup2ofEspiaccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group3of_espiaccess(&mut self) -> EnblReadGroup3ofEspiaccessW<PricIo108Spec> {
        EnblReadGroup3ofEspiaccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group4of_espiaccess(&mut self) -> EnblReadGroup4ofEspiaccessW<PricIo108Spec> {
        EnblReadGroup4ofEspiaccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of eSPI access"]
    #[inline(always)]
    pub fn enbl_read_group5of_espiaccess(&mut self) -> EnblReadGroup5ofEspiaccessW<PricIo108Spec> {
        EnblReadGroup5ofEspiaccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1108PRIC1_108\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1108pric11080500(
        &mut self,
    ) -> EnblRstToleranceOfPric1108pric11080500W<PricIo108Spec> {
        EnblRstToleranceOfPric1108pric11080500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1108PRIC1_108\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1108pric11080600(
        &mut self,
    ) -> EnblWrProtOfPric1108pric11080600W<PricIo108Spec> {
        EnblWrProtOfPric1108pric11080600W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo108Spec> {
        Reserved7W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo108Spec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo108Spec> {
        Reserved5W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo108Spec> {
        Reserved4W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo108Spec> {
        Reserved3W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo108Spec> {
        Reserved2W::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo108Spec> {
        Reserved1W::new(self, 14)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group0of_post_code_dmaaccess(
        &mut self,
    ) -> EnblReadGroup0ofPostCodeDmaaccessW<PricIo108Spec> {
        EnblReadGroup0ofPostCodeDmaaccessW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group1of_post_code_dmaaccess(
        &mut self,
    ) -> EnblReadGroup1ofPostCodeDmaaccessW<PricIo108Spec> {
        EnblReadGroup1ofPostCodeDmaaccessW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group2of_post_code_dmaaccess(
        &mut self,
    ) -> EnblReadGroup2ofPostCodeDmaaccessW<PricIo108Spec> {
        EnblReadGroup2ofPostCodeDmaaccessW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group3of_post_code_dmaaccess(
        &mut self,
    ) -> EnblReadGroup3ofPostCodeDmaaccessW<PricIo108Spec> {
        EnblReadGroup3ofPostCodeDmaaccessW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group4of_post_code_dmaaccess(
        &mut self,
    ) -> EnblReadGroup4ofPostCodeDmaaccessW<PricIo108Spec> {
        EnblReadGroup4ofPostCodeDmaaccessW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of PostCode DMA access"]
    #[inline(always)]
    pub fn enbl_read_group5of_post_code_dmaaccess(
        &mut self,
    ) -> EnblReadGroup5ofPostCodeDmaaccessW<PricIo108Spec> {
        EnblReadGroup5ofPostCodeDmaaccessW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1108PRIC1_108\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1108pric11082116(
        &mut self,
    ) -> EnblRstToleranceOfPric1108pric11082116W<PricIo108Spec> {
        EnblRstToleranceOfPric1108pric11082116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1108PRIC1_108\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1108pric11082216(
        &mut self,
    ) -> EnblWrProtOfPric1108pric11082216W<PricIo108Spec> {
        EnblWrProtOfPric1108pric11082216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group0of_uart_dbg_access(
        &mut self,
    ) -> EnblReadGroup0ofUartDbgAccessW<PricIo108Spec> {
        EnblReadGroup0ofUartDbgAccessW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group1of_uart_dbg_access(
        &mut self,
    ) -> EnblReadGroup1ofUartDbgAccessW<PricIo108Spec> {
        EnblReadGroup1ofUartDbgAccessW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group2of_uart_dbg_access(
        &mut self,
    ) -> EnblReadGroup2ofUartDbgAccessW<PricIo108Spec> {
        EnblReadGroup2ofUartDbgAccessW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group3of_uart_dbg_access(
        &mut self,
    ) -> EnblReadGroup3ofUartDbgAccessW<PricIo108Spec> {
        EnblReadGroup3ofUartDbgAccessW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group4of_uart_dbg_access(
        &mut self,
    ) -> EnblReadGroup4ofUartDbgAccessW<PricIo108Spec> {
        EnblReadGroup4ofUartDbgAccessW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of UartDbg access"]
    #[inline(always)]
    pub fn enbl_read_group5of_uart_dbg_access(
        &mut self,
    ) -> EnblReadGroup5ofUartDbgAccessW<PricIo108Spec> {
        EnblReadGroup5ofUartDbgAccessW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1108PRIC1_108\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1108pric11082924(
        &mut self,
    ) -> EnblRstToleranceOfPric1108pric11082924W<PricIo108Spec> {
        EnblRstToleranceOfPric1108pric11082924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1108PRIC1_108\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1108pric11083024(
        &mut self,
    ) -> EnblWrProtOfPric1108pric11083024W<PricIo108Spec> {
        EnblWrProtOfPric1108pric11083024W::new(self, 31)
    }
}
#[doc = "Master Read Group Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io108::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io108::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo108Spec;
impl crate::RegisterSpec for PricIo108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io108::R`](R) reader structure"]
impl crate::Readable for PricIo108Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io108::W`](W) writer structure"]
impl crate::Writable for PricIo108Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO108 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo108Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
