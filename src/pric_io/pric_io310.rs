#[doc = "Register `PRIC_IO310` reader"]
pub type R = crate::R<PricIo310Spec>;
#[doc = "Register `PRIC_IO310` writer"]
pub type W = crate::W<PricIo310Spec>;
#[doc = "Field `EnblReadGroup0OfUHCI` reader - Enable Read Group #0 of UHCI"]
pub type EnblReadGroup0ofUhciR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUHCI` writer - Enable Read Group #0 of UHCI"]
pub type EnblReadGroup0ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUHCI` reader - Enable Read Group #1 of UHCI"]
pub type EnblReadGroup1ofUhciR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUHCI` writer - Enable Read Group #1 of UHCI"]
pub type EnblReadGroup1ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUHCI` reader - Enable Read Group #2 of UHCI"]
pub type EnblReadGroup2ofUhciR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUHCI` writer - Enable Read Group #2 of UHCI"]
pub type EnblReadGroup2ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUHCI` reader - Enable Read Group #3 of UHCI"]
pub type EnblReadGroup3ofUhciR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUHCI` writer - Enable Read Group #3 of UHCI"]
pub type EnblReadGroup3ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUHCI` reader - Enable Read Group #4 of UHCI"]
pub type EnblReadGroup4ofUhciR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUHCI` writer - Enable Read Group #4 of UHCI"]
pub type EnblReadGroup4ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUHCI` reader - Enable Read Group #5 of UHCI"]
pub type EnblReadGroup5ofUhciR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUHCI` writer - Enable Read Group #5 of UHCI"]
pub type EnblReadGroup5ofUhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1310PRIC1_310\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1310pric13100500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1310pric13100500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1310pric13100500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1310PRIC13100500` reader - Enable Reset Tolerance of PRIC1310PRIC1_310\\[05:00\\]"]
pub type EnblRstToleranceOfPric1310pric13100500R =
    crate::BitReader<EnblRstToleranceOfPric1310pric13100500>;
impl EnblRstToleranceOfPric1310pric13100500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1310pric13100500 {
        match self.bits {
            false => EnblRstToleranceOfPric1310pric13100500::ResetBySrst,
            true => EnblRstToleranceOfPric1310pric13100500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1310pric13100500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1310pric13100500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1310PRIC13100500` writer - Enable Reset Tolerance of PRIC1310PRIC1_310\\[05:00\\]"]
pub type EnblRstToleranceOfPric1310pric13100500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1310pric13100500>;
impl<'a, REG> EnblRstToleranceOfPric1310pric13100500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1310pric13100500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1310pric13100500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1310PRIC13100600` reader - Enable Write Protection of PRIC1310PRIC1_310\\[06:00\\]"]
pub type EnblWrProtOfPric1310pric13100600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1310PRIC13100600` writer - Enable Write Protection of PRIC1310PRIC1_310\\[06:00\\]"]
pub type EnblWrProtOfPric1310pric13100600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUSB2PortC` reader - Enable Read Group #0 of USB2 Port C"]
pub type EnblReadGroup0ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUSB2PortC` writer - Enable Read Group #0 of USB2 Port C"]
pub type EnblReadGroup0ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUSB2PortC` reader - Enable Read Group #1 of USB2 Port C"]
pub type EnblReadGroup1ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUSB2PortC` writer - Enable Read Group #1 of USB2 Port C"]
pub type EnblReadGroup1ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUSB2PortC` reader - Enable Read Group #2 of USB2 Port C"]
pub type EnblReadGroup2ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUSB2PortC` writer - Enable Read Group #2 of USB2 Port C"]
pub type EnblReadGroup2ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUSB2PortC` reader - Enable Read Group #3 of USB2 Port C"]
pub type EnblReadGroup3ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUSB2PortC` writer - Enable Read Group #3 of USB2 Port C"]
pub type EnblReadGroup3ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUSB2PortC` reader - Enable Read Group #4 of USB2 Port C"]
pub type EnblReadGroup4ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUSB2PortC` writer - Enable Read Group #4 of USB2 Port C"]
pub type EnblReadGroup4ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUSB2PortC` reader - Enable Read Group #5 of USB2 Port C"]
pub type EnblReadGroup5ofUsb2portCR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUSB2PortC` writer - Enable Read Group #5 of USB2 Port C"]
pub type EnblReadGroup5ofUsb2portCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1310PRIC1_310\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1310pric13101308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1310pric13101308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1310pric13101308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1310PRIC13101308` reader - Enable Reset Tolerance of PRIC1310PRIC1_310\\[13:08\\]"]
pub type EnblRstToleranceOfPric1310pric13101308R =
    crate::BitReader<EnblRstToleranceOfPric1310pric13101308>;
impl EnblRstToleranceOfPric1310pric13101308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1310pric13101308 {
        match self.bits {
            false => EnblRstToleranceOfPric1310pric13101308::ResetBySrst,
            true => EnblRstToleranceOfPric1310pric13101308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1310pric13101308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1310pric13101308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1310PRIC13101308` writer - Enable Reset Tolerance of PRIC1310PRIC1_310\\[13:08\\]"]
pub type EnblRstToleranceOfPric1310pric13101308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1310pric13101308>;
impl<'a, REG> EnblRstToleranceOfPric1310pric13101308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1310pric13101308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1310pric13101308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1310PRIC13101408` reader - Enable Write Protection of PRIC1310PRIC1_310\\[14:08\\]"]
pub type EnblWrProtOfPric1310pric13101408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1310PRIC13101408` writer - Enable Write Protection of PRIC1310PRIC1_310\\[14:08\\]"]
pub type EnblWrProtOfPric1310pric13101408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUSB2PortD` reader - Enable Read Group #0 of USB2 Port D"]
pub type EnblReadGroup0ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUSB2PortD` writer - Enable Read Group #0 of USB2 Port D"]
pub type EnblReadGroup0ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUSB2PortD` reader - Enable Read Group #1 of USB2 Port D"]
pub type EnblReadGroup1ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUSB2PortD` writer - Enable Read Group #1 of USB2 Port D"]
pub type EnblReadGroup1ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUSB2PortD` reader - Enable Read Group #2 of USB2 Port D"]
pub type EnblReadGroup2ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUSB2PortD` writer - Enable Read Group #2 of USB2 Port D"]
pub type EnblReadGroup2ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUSB2PortD` reader - Enable Read Group #3 of USB2 Port D"]
pub type EnblReadGroup3ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUSB2PortD` writer - Enable Read Group #3 of USB2 Port D"]
pub type EnblReadGroup3ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUSB2PortD` reader - Enable Read Group #4 of USB2 Port D"]
pub type EnblReadGroup4ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUSB2PortD` writer - Enable Read Group #4 of USB2 Port D"]
pub type EnblReadGroup4ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUSB2PortD` reader - Enable Read Group #5 of USB2 Port D"]
pub type EnblReadGroup5ofUsb2portDR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUSB2PortD` writer - Enable Read Group #5 of USB2 Port D"]
pub type EnblReadGroup5ofUsb2portDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1310PRIC1_310\\[21:16\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1310pric13102116 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1310pric13102116> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1310pric13102116) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1310PRIC13102116` reader - Enable Reset Tolerance of PRIC1310PRIC1_310\\[21:16\\]"]
pub type EnblRstToleranceOfPric1310pric13102116R =
    crate::BitReader<EnblRstToleranceOfPric1310pric13102116>;
impl EnblRstToleranceOfPric1310pric13102116R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1310pric13102116 {
        match self.bits {
            false => EnblRstToleranceOfPric1310pric13102116::ResetBySrst,
            true => EnblRstToleranceOfPric1310pric13102116::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1310pric13102116::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1310pric13102116::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1310PRIC13102116` writer - Enable Reset Tolerance of PRIC1310PRIC1_310\\[21:16\\]"]
pub type EnblRstToleranceOfPric1310pric13102116W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1310pric13102116>;
impl<'a, REG> EnblRstToleranceOfPric1310pric13102116W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1310pric13102116::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1310pric13102116::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1310PRIC13102216` reader - Enable Write Protection of PRIC1310PRIC1_310\\[22:16\\]"]
pub type EnblWrProtOfPric1310pric13102216R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1310PRIC13102216` writer - Enable Write Protection of PRIC1310PRIC1_310\\[22:16\\]"]
pub type EnblWrProtOfPric1310pric13102216W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfINTC` reader - Enable Read Group #0 of INTC"]
pub type EnblReadGroup0ofIntcR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfINTC` writer - Enable Read Group #0 of INTC"]
pub type EnblReadGroup0ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfINTC` reader - Enable Read Group #1 of INTC"]
pub type EnblReadGroup1ofIntcR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfINTC` writer - Enable Read Group #1 of INTC"]
pub type EnblReadGroup1ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfINTC` reader - Enable Read Group #2 of INTC"]
pub type EnblReadGroup2ofIntcR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfINTC` writer - Enable Read Group #2 of INTC"]
pub type EnblReadGroup2ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfINTC` reader - Enable Read Group #3 of INTC"]
pub type EnblReadGroup3ofIntcR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfINTC` writer - Enable Read Group #3 of INTC"]
pub type EnblReadGroup3ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfINTC` reader - Enable Read Group #4 of INTC"]
pub type EnblReadGroup4ofIntcR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfINTC` writer - Enable Read Group #4 of INTC"]
pub type EnblReadGroup4ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfINTC` reader - Enable Read Group #5 of INTC"]
pub type EnblReadGroup5ofIntcR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfINTC` writer - Enable Read Group #5 of INTC"]
pub type EnblReadGroup5ofIntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1310PRIC1_310\\[29:24\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1310pric13102924 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1310pric13102924> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1310pric13102924) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1310PRIC13102924` reader - Enable Reset Tolerance of PRIC1310PRIC1_310\\[29:24\\]"]
pub type EnblRstToleranceOfPric1310pric13102924R =
    crate::BitReader<EnblRstToleranceOfPric1310pric13102924>;
impl EnblRstToleranceOfPric1310pric13102924R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1310pric13102924 {
        match self.bits {
            false => EnblRstToleranceOfPric1310pric13102924::ResetBySrst,
            true => EnblRstToleranceOfPric1310pric13102924::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1310pric13102924::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1310pric13102924::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1310PRIC13102924` writer - Enable Reset Tolerance of PRIC1310PRIC1_310\\[29:24\\]"]
pub type EnblRstToleranceOfPric1310pric13102924W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1310pric13102924>;
impl<'a, REG> EnblRstToleranceOfPric1310pric13102924W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1310pric13102924::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1310pric13102924::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1310PRIC13103024` reader - Enable Write Protection of PRIC1310PRIC1_310\\[30:24\\]"]
pub type EnblWrProtOfPric1310pric13103024R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1310PRIC13103024` writer - Enable Write Protection of PRIC1310PRIC1_310\\[30:24\\]"]
pub type EnblWrProtOfPric1310pric13103024W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Read Group #0 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group0of_uhci(&self) -> EnblReadGroup0ofUhciR {
        EnblReadGroup0ofUhciR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group1of_uhci(&self) -> EnblReadGroup1ofUhciR {
        EnblReadGroup1ofUhciR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group2of_uhci(&self) -> EnblReadGroup2ofUhciR {
        EnblReadGroup2ofUhciR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group3of_uhci(&self) -> EnblReadGroup3ofUhciR {
        EnblReadGroup3ofUhciR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group4of_uhci(&self) -> EnblReadGroup4ofUhciR {
        EnblReadGroup4ofUhciR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group5of_uhci(&self) -> EnblReadGroup5ofUhciR {
        EnblReadGroup5ofUhciR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1310PRIC1_310\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1310pric13100500(
        &self,
    ) -> EnblRstToleranceOfPric1310pric13100500R {
        EnblRstToleranceOfPric1310pric13100500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1310PRIC1_310\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1310pric13100600(&self) -> EnblWrProtOfPric1310pric13100600R {
        EnblWrProtOfPric1310pric13100600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group0of_usb2port_c(&self) -> EnblReadGroup0ofUsb2portCR {
        EnblReadGroup0ofUsb2portCR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group1of_usb2port_c(&self) -> EnblReadGroup1ofUsb2portCR {
        EnblReadGroup1ofUsb2portCR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group2of_usb2port_c(&self) -> EnblReadGroup2ofUsb2portCR {
        EnblReadGroup2ofUsb2portCR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group3of_usb2port_c(&self) -> EnblReadGroup3ofUsb2portCR {
        EnblReadGroup3ofUsb2portCR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group4of_usb2port_c(&self) -> EnblReadGroup4ofUsb2portCR {
        EnblReadGroup4ofUsb2portCR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group5of_usb2port_c(&self) -> EnblReadGroup5ofUsb2portCR {
        EnblReadGroup5ofUsb2portCR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1310PRIC1_310\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1310pric13101308(
        &self,
    ) -> EnblRstToleranceOfPric1310pric13101308R {
        EnblRstToleranceOfPric1310pric13101308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1310PRIC1_310\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1310pric13101408(&self) -> EnblWrProtOfPric1310pric13101408R {
        EnblWrProtOfPric1310pric13101408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group0of_usb2port_d(&self) -> EnblReadGroup0ofUsb2portDR {
        EnblReadGroup0ofUsb2portDR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group1of_usb2port_d(&self) -> EnblReadGroup1ofUsb2portDR {
        EnblReadGroup1ofUsb2portDR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group2of_usb2port_d(&self) -> EnblReadGroup2ofUsb2portDR {
        EnblReadGroup2ofUsb2portDR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group3of_usb2port_d(&self) -> EnblReadGroup3ofUsb2portDR {
        EnblReadGroup3ofUsb2portDR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group4of_usb2port_d(&self) -> EnblReadGroup4ofUsb2portDR {
        EnblReadGroup4ofUsb2portDR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group5of_usb2port_d(&self) -> EnblReadGroup5ofUsb2portDR {
        EnblReadGroup5ofUsb2portDR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1310PRIC1_310\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1310pric13102116(
        &self,
    ) -> EnblRstToleranceOfPric1310pric13102116R {
        EnblRstToleranceOfPric1310pric13102116R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1310PRIC1_310\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1310pric13102216(&self) -> EnblWrProtOfPric1310pric13102216R {
        EnblWrProtOfPric1310pric13102216R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group0of_intc(&self) -> EnblReadGroup0ofIntcR {
        EnblReadGroup0ofIntcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group1of_intc(&self) -> EnblReadGroup1ofIntcR {
        EnblReadGroup1ofIntcR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group2of_intc(&self) -> EnblReadGroup2ofIntcR {
        EnblReadGroup2ofIntcR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group3of_intc(&self) -> EnblReadGroup3ofIntcR {
        EnblReadGroup3ofIntcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group4of_intc(&self) -> EnblReadGroup4ofIntcR {
        EnblReadGroup4ofIntcR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group5of_intc(&self) -> EnblReadGroup5ofIntcR {
        EnblReadGroup5ofIntcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1310PRIC1_310\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1310pric13102924(
        &self,
    ) -> EnblRstToleranceOfPric1310pric13102924R {
        EnblRstToleranceOfPric1310pric13102924R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1310PRIC1_310\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1310pric13103024(&self) -> EnblWrProtOfPric1310pric13103024R {
        EnblWrProtOfPric1310pric13103024R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Read Group #0 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group0of_uhci(&mut self) -> EnblReadGroup0ofUhciW<PricIo310Spec> {
        EnblReadGroup0ofUhciW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group1of_uhci(&mut self) -> EnblReadGroup1ofUhciW<PricIo310Spec> {
        EnblReadGroup1ofUhciW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group2of_uhci(&mut self) -> EnblReadGroup2ofUhciW<PricIo310Spec> {
        EnblReadGroup2ofUhciW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group3of_uhci(&mut self) -> EnblReadGroup3ofUhciW<PricIo310Spec> {
        EnblReadGroup3ofUhciW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group4of_uhci(&mut self) -> EnblReadGroup4ofUhciW<PricIo310Spec> {
        EnblReadGroup4ofUhciW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of UHCI"]
    #[inline(always)]
    pub fn enbl_read_group5of_uhci(&mut self) -> EnblReadGroup5ofUhciW<PricIo310Spec> {
        EnblReadGroup5ofUhciW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1310PRIC1_310\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1310pric13100500(
        &mut self,
    ) -> EnblRstToleranceOfPric1310pric13100500W<PricIo310Spec> {
        EnblRstToleranceOfPric1310pric13100500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1310PRIC1_310\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1310pric13100600(
        &mut self,
    ) -> EnblWrProtOfPric1310pric13100600W<PricIo310Spec> {
        EnblWrProtOfPric1310pric13100600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group0of_usb2port_c(&mut self) -> EnblReadGroup0ofUsb2portCW<PricIo310Spec> {
        EnblReadGroup0ofUsb2portCW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group1of_usb2port_c(&mut self) -> EnblReadGroup1ofUsb2portCW<PricIo310Spec> {
        EnblReadGroup1ofUsb2portCW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group2of_usb2port_c(&mut self) -> EnblReadGroup2ofUsb2portCW<PricIo310Spec> {
        EnblReadGroup2ofUsb2portCW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group3of_usb2port_c(&mut self) -> EnblReadGroup3ofUsb2portCW<PricIo310Spec> {
        EnblReadGroup3ofUsb2portCW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group4of_usb2port_c(&mut self) -> EnblReadGroup4ofUsb2portCW<PricIo310Spec> {
        EnblReadGroup4ofUsb2portCW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of USB2 Port C"]
    #[inline(always)]
    pub fn enbl_read_group5of_usb2port_c(&mut self) -> EnblReadGroup5ofUsb2portCW<PricIo310Spec> {
        EnblReadGroup5ofUsb2portCW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1310PRIC1_310\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1310pric13101308(
        &mut self,
    ) -> EnblRstToleranceOfPric1310pric13101308W<PricIo310Spec> {
        EnblRstToleranceOfPric1310pric13101308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1310PRIC1_310\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1310pric13101408(
        &mut self,
    ) -> EnblWrProtOfPric1310pric13101408W<PricIo310Spec> {
        EnblWrProtOfPric1310pric13101408W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Read Group #0 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group0of_usb2port_d(&mut self) -> EnblReadGroup0ofUsb2portDW<PricIo310Spec> {
        EnblReadGroup0ofUsb2portDW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Read Group #1 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group1of_usb2port_d(&mut self) -> EnblReadGroup1ofUsb2portDW<PricIo310Spec> {
        EnblReadGroup1ofUsb2portDW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Read Group #2 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group2of_usb2port_d(&mut self) -> EnblReadGroup2ofUsb2portDW<PricIo310Spec> {
        EnblReadGroup2ofUsb2portDW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Read Group #3 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group3of_usb2port_d(&mut self) -> EnblReadGroup3ofUsb2portDW<PricIo310Spec> {
        EnblReadGroup3ofUsb2portDW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Read Group #4 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group4of_usb2port_d(&mut self) -> EnblReadGroup4ofUsb2portDW<PricIo310Spec> {
        EnblReadGroup4ofUsb2portDW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Read Group #5 of USB2 Port D"]
    #[inline(always)]
    pub fn enbl_read_group5of_usb2port_d(&mut self) -> EnblReadGroup5ofUsb2portDW<PricIo310Spec> {
        EnblReadGroup5ofUsb2portDW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Reset Tolerance of PRIC1310PRIC1_310\\[21:16\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1310pric13102116(
        &mut self,
    ) -> EnblRstToleranceOfPric1310pric13102116W<PricIo310Spec> {
        EnblRstToleranceOfPric1310pric13102116W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of PRIC1310PRIC1_310\\[22:16\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1310pric13102216(
        &mut self,
    ) -> EnblWrProtOfPric1310pric13102216W<PricIo310Spec> {
        EnblWrProtOfPric1310pric13102216W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Read Group #0 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group0of_intc(&mut self) -> EnblReadGroup0ofIntcW<PricIo310Spec> {
        EnblReadGroup0ofIntcW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Read Group #1 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group1of_intc(&mut self) -> EnblReadGroup1ofIntcW<PricIo310Spec> {
        EnblReadGroup1ofIntcW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable Read Group #2 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group2of_intc(&mut self) -> EnblReadGroup2ofIntcW<PricIo310Spec> {
        EnblReadGroup2ofIntcW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Read Group #3 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group3of_intc(&mut self) -> EnblReadGroup3ofIntcW<PricIo310Spec> {
        EnblReadGroup3ofIntcW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Read Group #4 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group4of_intc(&mut self) -> EnblReadGroup4ofIntcW<PricIo310Spec> {
        EnblReadGroup4ofIntcW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Read Group #5 of INTC"]
    #[inline(always)]
    pub fn enbl_read_group5of_intc(&mut self) -> EnblReadGroup5ofIntcW<PricIo310Spec> {
        EnblReadGroup5ofIntcW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Reset Tolerance of PRIC1310PRIC1_310\\[29:24\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1310pric13102924(
        &mut self,
    ) -> EnblRstToleranceOfPric1310pric13102924W<PricIo310Spec> {
        EnblRstToleranceOfPric1310pric13102924W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Write Protection of PRIC1310PRIC1_310\\[30:24\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1310pric13103024(
        &mut self,
    ) -> EnblWrProtOfPric1310pric13103024W<PricIo310Spec> {
        EnblWrProtOfPric1310pric13103024W::new(self, 31)
    }
}
#[doc = "Slave Read Group Setting Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io310::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io310::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo310Spec;
impl crate::RegisterSpec for PricIo310Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io310::R`](R) reader structure"]
impl crate::Readable for PricIo310Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io310::W`](W) writer structure"]
impl crate::Writable for PricIo310Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO310 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo310Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
