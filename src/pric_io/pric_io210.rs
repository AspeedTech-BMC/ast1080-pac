#[doc = "Register `PRIC_IO210` reader"]
pub type R = crate::R<PricIo210Spec>;
#[doc = "Register `PRIC_IO210` writer"]
pub type W = crate::W<PricIo210Spec>;
#[doc = "Field `EnblWrGroup0OfUHCI` reader - Enable Write Group #0 of UHCI"]
pub type EnblWrGroup0ofUhciR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUHCI` writer - Enable Write Group #0 of UHCI"]
pub type EnblWrGroup0ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUHCI` reader - Enable Write Group #1 of UHCI"]
pub type EnblWrGroup1ofUhciR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUHCI` writer - Enable Write Group #1 of UHCI"]
pub type EnblWrGroup1ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUHCI` reader - Enable Write Group #2 of UHCI"]
pub type EnblWrGroup2ofUhciR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUHCI` writer - Enable Write Group #2 of UHCI"]
pub type EnblWrGroup2ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUHCI` reader - Enable Write Group #3 of UHCI"]
pub type EnblWrGroup3ofUhciR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUHCI` writer - Enable Write Group #3 of UHCI"]
pub type EnblWrGroup3ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUHCI` reader - Enable Write Group #4 of UHCI"]
pub type EnblWrGroup4ofUhciR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUHCI` writer - Enable Write Group #4 of UHCI"]
pub type EnblWrGroup4ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUHCI` reader - Enable Write Group #5 of UHCI"]
pub type EnblWrGroup5ofUhciR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUHCI` writer - Enable Write Group #5 of UHCI"]
pub type EnblWrGroup5ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1210PRIC1_210\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1210pric12100500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1210pric12100500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1210pric12100500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1210PRIC12100500` reader - Enable Reset Tolerance of PRIC1210PRIC1_210\\[05:00\\]"]
pub type EnblRstToleranceOfPric1210pric12100500R =
    crate::BitReader<EnblRstToleranceOfPric1210pric12100500>;
impl EnblRstToleranceOfPric1210pric12100500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1210pric12100500 {
        match self.bits {
            false => EnblRstToleranceOfPric1210pric12100500::ResetBySrst,
            true => EnblRstToleranceOfPric1210pric12100500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1210pric12100500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1210pric12100500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1210PRIC12100500` writer - Enable Reset Tolerance of PRIC1210PRIC1_210\\[05:00\\]"]
pub type EnblRstToleranceOfPric1210pric12100500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1210pric12100500>;
impl<'a, REG> EnblRstToleranceOfPric1210pric12100500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1210pric12100500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1210pric12100500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1210PRIC12100600` reader - Enable Write Protection of PRIC1210PRIC1_210\\[06:00\\]"]
pub type EnblWrProtOfPric1210pric12100600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1210PRIC12100600` writer - Enable Write Protection of PRIC1210PRIC1_210\\[06:00\\]"]
pub type EnblWrProtOfPric1210pric12100600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUSB2PortC` reader - Enable Write Group #0 of USB2 Port C"]
pub type EnblWrGroup0ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUSB2PortC` writer - Enable Write Group #0 of USB2 Port C"]
pub type EnblWrGroup0ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUSB2PortC` reader - Enable Write Group #1 of USB2 Port C"]
pub type EnblWrGroup1ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUSB2PortC` writer - Enable Write Group #1 of USB2 Port C"]
pub type EnblWrGroup1ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUSB2PortC` reader - Enable Write Group #2 of USB2 Port C"]
pub type EnblWrGroup2ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUSB2PortC` writer - Enable Write Group #2 of USB2 Port C"]
pub type EnblWrGroup2ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUSB2PortC` reader - Enable Write Group #3 of USB2 Port C"]
pub type EnblWrGroup3ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUSB2PortC` writer - Enable Write Group #3 of USB2 Port C"]
pub type EnblWrGroup3ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUSB2PortC` reader - Enable Write Group #4 of USB2 Port C"]
pub type EnblWrGroup4ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUSB2PortC` writer - Enable Write Group #4 of USB2 Port C"]
pub type EnblWrGroup4ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUSB2PortC` reader - Enable Write Group #5 of USB2 Port C"]
pub type EnblWrGroup5ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUSB2PortC` writer - Enable Write Group #5 of USB2 Port C"]
pub type EnblWrGroup5ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1210PRIC1_210\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1210pric12101308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1210pric12101308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1210pric12101308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1210PRIC12101308` reader - Enable Reset Tolerance of PRIC1210PRIC1_210\\[13:08\\]"]
pub type EnblRstToleranceOfPric1210pric12101308R =
    crate::BitReader<EnblRstToleranceOfPric1210pric12101308>;
impl EnblRstToleranceOfPric1210pric12101308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1210pric12101308 {
        match self.bits {
            false => EnblRstToleranceOfPric1210pric12101308::ResetBySrst,
            true => EnblRstToleranceOfPric1210pric12101308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1210pric12101308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1210pric12101308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1210PRIC12101308` writer - Enable Reset Tolerance of PRIC1210PRIC1_210\\[13:08\\]"]
pub type EnblRstToleranceOfPric1210pric12101308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1210pric12101308>;
impl<'a, REG> EnblRstToleranceOfPric1210pric12101308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1210pric12101308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1210pric12101308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1210PRIC12101408` reader - Enable Write Protection of PRIC1210PRIC1_210\\[14:08\\]"]
pub type EnblWrProtOfPric1210pric12101408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1210PRIC12101408` writer - Enable Write Protection of PRIC1210PRIC1_210\\[14:08\\]"]
pub type EnblWrProtOfPric1210pric12101408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUSB2PortD` reader - Enable Write Group #0 of USB2 Port D"]
pub type EnblWrGroup0ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUSB2PortD` writer - Enable Write Group #0 of USB2 Port D"]
pub type EnblWrGroup0ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUSB2PortD` reader - Enable Write Group #1 of USB2 Port D"]
pub type EnblWrGroup1ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUSB2PortD` writer - Enable Write Group #1 of USB2 Port D"]
pub type EnblWrGroup1ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUSB2PortD` reader - Enable Write Group #2 of USB2 Port D"]
pub type EnblWrGroup2ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUSB2PortD` writer - Enable Write Group #2 of USB2 Port D"]
pub type EnblWrGroup2ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUSB2PortD` reader - Enable Write Group #3 of USB2 Port D"]
pub type EnblWrGroup3ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUSB2PortD` writer - Enable Write Group #3 of USB2 Port D"]
pub type EnblWrGroup3ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUSB2PortD` reader - Enable Write Group #4 of USB2 Port D"]
pub type EnblWrGroup4ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUSB2PortD` writer - Enable Write Group #4 of USB2 Port D"]
pub type EnblWrGroup4ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUSB2PortD` reader - Enable Write Group #5 of USB2 Port D"]
pub type EnblWrGroup5ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUSB2PortD` writer - Enable Write Group #5 of USB2 Port D"]
pub type EnblWrGroup5ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1210PRIC1_210\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1210pric12102116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1210pric12102116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1210pric12102116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1210PRIC12102116` reader - Enable Reset Tolerance of PRIC1210PRIC1_210\\[21:16\\]"]
pub type EnblRstToleranceOfPric1210pric12102116R =
    crate::BitReader<EnblRstToleranceOfPric1210pric12102116>;
impl EnblRstToleranceOfPric1210pric12102116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1210pric12102116 {
        match self.bits {
            false => EnblRstToleranceOfPric1210pric12102116::ResetBySrst,
            true => EnblRstToleranceOfPric1210pric12102116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1210pric12102116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1210pric12102116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1210PRIC12102116` writer - Enable Reset Tolerance of PRIC1210PRIC1_210\\[21:16\\]"]
pub type EnblRstToleranceOfPric1210pric12102116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1210pric12102116>;
impl<'a, REG> EnblRstToleranceOfPric1210pric12102116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1210pric12102116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1210pric12102116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1210PRIC12102216` reader - Enable Write Protection of PRIC1210PRIC1_210\\[22:16\\]"]
pub type EnblWrProtOfPric1210pric12102216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1210PRIC12102216` writer - Enable Write Protection of PRIC1210PRIC1_210\\[22:16\\]"]
pub type EnblWrProtOfPric1210pric12102216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfINTC` reader - Enable Write Group #0 of INTC"]
pub type EnblWrGroup0ofIntcR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfINTC` writer - Enable Write Group #0 of INTC"]
pub type EnblWrGroup0ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfINTC` reader - Enable Write Group #1 of INTC"]
pub type EnblWrGroup1ofIntcR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfINTC` writer - Enable Write Group #1 of INTC"]
pub type EnblWrGroup1ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfINTC` reader - Enable Write Group #2 of INTC"]
pub type EnblWrGroup2ofIntcR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfINTC` writer - Enable Write Group #2 of INTC"]
pub type EnblWrGroup2ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfINTC` reader - Enable Write Group #3 of INTC"]
pub type EnblWrGroup3ofIntcR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfINTC` writer - Enable Write Group #3 of INTC"]
pub type EnblWrGroup3ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfINTC` reader - Enable Write Group #4 of INTC"]
pub type EnblWrGroup4ofIntcR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfINTC` writer - Enable Write Group #4 of INTC"]
pub type EnblWrGroup4ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfINTC` reader - Enable Write Group #5 of INTC"]
pub type EnblWrGroup5ofIntcR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfINTC` writer - Enable Write Group #5 of INTC"]
pub type EnblWrGroup5ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1210PRIC1_210\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1210pric12102924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1210pric12102924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1210pric12102924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1210PRIC12102924` reader - Enable Reset Tolerance of PRIC1210PRIC1_210\\[29:24\\]"]
pub type EnblRstToleranceOfPric1210pric12102924R =
    crate::BitReader<EnblRstToleranceOfPric1210pric12102924>;
impl EnblRstToleranceOfPric1210pric12102924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1210pric12102924 {
        match self.bits {
            false => EnblRstToleranceOfPric1210pric12102924::ResetBySrst,
            true => EnblRstToleranceOfPric1210pric12102924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1210pric12102924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1210pric12102924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1210PRIC12102924` writer - Enable Reset Tolerance of PRIC1210PRIC1_210\\[29:24\\]"]
pub type EnblRstToleranceOfPric1210pric12102924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1210pric12102924>;
impl<'a, REG> EnblRstToleranceOfPric1210pric12102924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1210pric12102924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1210pric12102924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1210PRIC12103024` reader - Enable Write Protection of PRIC1210PRIC1_210\\[30:24\\]"]
pub type EnblWrProtOfPric1210pric12103024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1210PRIC12103024` writer - Enable Write Protection of PRIC1210PRIC1_210\\[30:24\\]"]
pub type EnblWrProtOfPric1210pric12103024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uhci(&self) -> EnblWrGroup0ofUhciR {
        EnblWrGroup0ofUhciR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uhci(&self) -> EnblWrGroup1ofUhciR {
        EnblWrGroup1ofUhciR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uhci(&self) -> EnblWrGroup2ofUhciR {
        EnblWrGroup2ofUhciR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uhci(&self) -> EnblWrGroup3ofUhciR {
        EnblWrGroup3ofUhciR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uhci(&self) -> EnblWrGroup4ofUhciR {
        EnblWrGroup4ofUhciR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uhci(&self) -> EnblWrGroup5ofUhciR {
        EnblWrGroup5ofUhciR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1210PRIC1_210\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1210pric12100500(
        &self,
    ) -> EnblRstToleranceOfPric1210pric12100500R {
        EnblRstToleranceOfPric1210pric12100500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1210PRIC1_210\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1210pric12100600(&self) -> EnblWrProtOfPric1210pric12100600R {
        EnblWrProtOfPric1210pric12100600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group0of_usb2port_c(&self) -> EnblWrGroup0ofUsb2portCR {
        EnblWrGroup0ofUsb2portCR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group1of_usb2port_c(&self) -> EnblWrGroup1ofUsb2portCR {
        EnblWrGroup1ofUsb2portCR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group2of_usb2port_c(&self) -> EnblWrGroup2ofUsb2portCR {
        EnblWrGroup2ofUsb2portCR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group3of_usb2port_c(&self) -> EnblWrGroup3ofUsb2portCR {
        EnblWrGroup3ofUsb2portCR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group4of_usb2port_c(&self) -> EnblWrGroup4ofUsb2portCR {
        EnblWrGroup4ofUsb2portCR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group5of_usb2port_c(&self) -> EnblWrGroup5ofUsb2portCR {
        EnblWrGroup5ofUsb2portCR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1210PRIC1_210\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1210pric12101308(
        &self,
    ) -> EnblRstToleranceOfPric1210pric12101308R {
        EnblRstToleranceOfPric1210pric12101308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1210PRIC1_210\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1210pric12101408(&self) -> EnblWrProtOfPric1210pric12101408R {
        EnblWrProtOfPric1210pric12101408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group0of_usb2port_d(&self) -> EnblWrGroup0ofUsb2portDR {
        EnblWrGroup0ofUsb2portDR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group1of_usb2port_d(&self) -> EnblWrGroup1ofUsb2portDR {
        EnblWrGroup1ofUsb2portDR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group2of_usb2port_d(&self) -> EnblWrGroup2ofUsb2portDR {
        EnblWrGroup2ofUsb2portDR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group3of_usb2port_d(&self) -> EnblWrGroup3ofUsb2portDR {
        EnblWrGroup3ofUsb2portDR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group4of_usb2port_d(&self) -> EnblWrGroup4ofUsb2portDR {
        EnblWrGroup4ofUsb2portDR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group5of_usb2port_d(&self) -> EnblWrGroup5ofUsb2portDR {
        EnblWrGroup5ofUsb2portDR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1210PRIC1_210\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1210pric12102116(
        &self,
    ) -> EnblRstToleranceOfPric1210pric12102116R {
        EnblRstToleranceOfPric1210pric12102116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1210PRIC1_210\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1210pric12102216(&self) -> EnblWrProtOfPric1210pric12102216R {
        EnblWrProtOfPric1210pric12102216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group0of_intc(&self) -> EnblWrGroup0ofIntcR {
        EnblWrGroup0ofIntcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group1of_intc(&self) -> EnblWrGroup1ofIntcR {
        EnblWrGroup1ofIntcR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group2of_intc(&self) -> EnblWrGroup2ofIntcR {
        EnblWrGroup2ofIntcR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group3of_intc(&self) -> EnblWrGroup3ofIntcR {
        EnblWrGroup3ofIntcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group4of_intc(&self) -> EnblWrGroup4ofIntcR {
        EnblWrGroup4ofIntcR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group5of_intc(&self) -> EnblWrGroup5ofIntcR {
        EnblWrGroup5ofIntcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1210PRIC1_210\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1210pric12102924(
        &self,
    ) -> EnblRstToleranceOfPric1210pric12102924R {
        EnblRstToleranceOfPric1210pric12102924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1210PRIC1_210\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1210pric12103024(&self) -> EnblWrProtOfPric1210pric12103024R {
        EnblWrProtOfPric1210pric12103024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group0of_uhci(&mut self) -> EnblWrGroup0ofUhciW<PricIo210Spec> {
        EnblWrGroup0ofUhciW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group1of_uhci(&mut self) -> EnblWrGroup1ofUhciW<PricIo210Spec> {
        EnblWrGroup1ofUhciW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group2of_uhci(&mut self) -> EnblWrGroup2ofUhciW<PricIo210Spec> {
        EnblWrGroup2ofUhciW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group3of_uhci(&mut self) -> EnblWrGroup3ofUhciW<PricIo210Spec> {
        EnblWrGroup3ofUhciW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group4of_uhci(&mut self) -> EnblWrGroup4ofUhciW<PricIo210Spec> {
        EnblWrGroup4ofUhciW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of UHCI"]
    #[inline(always)]
    pub fn enbl_wr_group5of_uhci(&mut self) -> EnblWrGroup5ofUhciW<PricIo210Spec> {
        EnblWrGroup5ofUhciW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1210PRIC1_210\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1210pric12100500(
        &mut self,
    ) -> EnblRstToleranceOfPric1210pric12100500W<PricIo210Spec> {
        EnblRstToleranceOfPric1210pric12100500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1210PRIC1_210\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1210pric12100600(
        &mut self,
    ) -> EnblWrProtOfPric1210pric12100600W<PricIo210Spec> {
        EnblWrProtOfPric1210pric12100600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group0of_usb2port_c(&mut self) -> EnblWrGroup0ofUsb2portCW<PricIo210Spec> {
        EnblWrGroup0ofUsb2portCW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group1of_usb2port_c(&mut self) -> EnblWrGroup1ofUsb2portCW<PricIo210Spec> {
        EnblWrGroup1ofUsb2portCW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group2of_usb2port_c(&mut self) -> EnblWrGroup2ofUsb2portCW<PricIo210Spec> {
        EnblWrGroup2ofUsb2portCW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group3of_usb2port_c(&mut self) -> EnblWrGroup3ofUsb2portCW<PricIo210Spec> {
        EnblWrGroup3ofUsb2portCW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group4of_usb2port_c(&mut self) -> EnblWrGroup4ofUsb2portCW<PricIo210Spec> {
        EnblWrGroup4ofUsb2portCW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_wr_group5of_usb2port_c(&mut self) -> EnblWrGroup5ofUsb2portCW<PricIo210Spec> {
        EnblWrGroup5ofUsb2portCW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1210PRIC1_210\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1210pric12101308(
        &mut self,
    ) -> EnblRstToleranceOfPric1210pric12101308W<PricIo210Spec> {
        EnblRstToleranceOfPric1210pric12101308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1210PRIC1_210\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1210pric12101408(
        &mut self,
    ) -> EnblWrProtOfPric1210pric12101408W<PricIo210Spec> {
        EnblWrProtOfPric1210pric12101408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Write Group #0 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group0of_usb2port_d(&mut self) -> EnblWrGroup0ofUsb2portDW<PricIo210Spec> {
        EnblWrGroup0ofUsb2portDW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Write Group #1 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group1of_usb2port_d(&mut self) -> EnblWrGroup1ofUsb2portDW<PricIo210Spec> {
        EnblWrGroup1ofUsb2portDW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Write Group #2 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group2of_usb2port_d(&mut self) -> EnblWrGroup2ofUsb2portDW<PricIo210Spec> {
        EnblWrGroup2ofUsb2portDW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Group #3 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group3of_usb2port_d(&mut self) -> EnblWrGroup3ofUsb2portDW<PricIo210Spec> {
        EnblWrGroup3ofUsb2portDW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Group #4 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group4of_usb2port_d(&mut self) -> EnblWrGroup4ofUsb2portDW<PricIo210Spec> {
        EnblWrGroup4ofUsb2portDW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Group #5 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_wr_group5of_usb2port_d(&mut self) -> EnblWrGroup5ofUsb2portDW<PricIo210Spec> {
        EnblWrGroup5ofUsb2portDW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1210PRIC1_210\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1210pric12102116(
        &mut self,
    ) -> EnblRstToleranceOfPric1210pric12102116W<PricIo210Spec> {
        EnblRstToleranceOfPric1210pric12102116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1210PRIC1_210\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1210pric12102216(
        &mut self,
    ) -> EnblWrProtOfPric1210pric12102216W<PricIo210Spec> {
        EnblWrProtOfPric1210pric12102216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Group #0 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group0of_intc(&mut self) -> EnblWrGroup0ofIntcW<PricIo210Spec> {
        EnblWrGroup0ofIntcW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Group #1 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group1of_intc(&mut self) -> EnblWrGroup1ofIntcW<PricIo210Spec> {
        EnblWrGroup1ofIntcW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Write Group #2 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group2of_intc(&mut self) -> EnblWrGroup2ofIntcW<PricIo210Spec> {
        EnblWrGroup2ofIntcW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Write Group #3 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group3of_intc(&mut self) -> EnblWrGroup3ofIntcW<PricIo210Spec> {
        EnblWrGroup3ofIntcW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Write Group #4 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group4of_intc(&mut self) -> EnblWrGroup4ofIntcW<PricIo210Spec> {
        EnblWrGroup4ofIntcW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Write Group #5 of INTC"]
    #[inline(always)]
    pub fn enbl_wr_group5of_intc(&mut self) -> EnblWrGroup5ofIntcW<PricIo210Spec> {
        EnblWrGroup5ofIntcW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1210PRIC1_210\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1210pric12102924(
        &mut self,
    ) -> EnblRstToleranceOfPric1210pric12102924W<PricIo210Spec> {
        EnblRstToleranceOfPric1210pric12102924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1210PRIC1_210\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1210pric12103024(
        &mut self,
    ) -> EnblWrProtOfPric1210pric12103024W<PricIo210Spec> {
        EnblWrProtOfPric1210pric12103024W::new(self, 31)
    }
}
#[doc = "Slave Write Group Setting Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io210::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io210::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo210Spec;
impl crate::RegisterSpec for PricIo210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io210::R`](R) reader structure"]
impl crate::Readable for PricIo210Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io210::W`](W) writer structure"]
impl crate::Writable for PricIo210Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO210 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo210Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
