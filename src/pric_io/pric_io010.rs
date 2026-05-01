#[doc = "Register `PRIC_IO010` reader"]
pub type R = crate::R<PricIo010Spec>;
#[doc = "Register `PRIC_IO010` writer"]
pub type W = crate::W<PricIo010Spec>;
#[doc = "Field `EnblWrGroup0OfUSB2PortAAccess` reader - Enable Write Group #0 of USB2 Port A access"]
pub type EnblWrGroup0ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUSB2PortAAccess` writer - Enable Write Group #0 of USB2 Port A access"]
pub type EnblWrGroup0ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUSB2PortAAccess` reader - Enable Write Group #1 of USB2 Port A access"]
pub type EnblWrGroup1ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUSB2PortAAccess` writer - Enable Write Group #1 of USB2 Port A access"]
pub type EnblWrGroup1ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUSB2PortAAccess` reader - Enable Write Group #2 of USB2 Port A access"]
pub type EnblWrGroup2ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUSB2PortAAccess` writer - Enable Write Group #2 of USB2 Port A access"]
pub type EnblWrGroup2ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUSB2PortAAccess` reader - Enable Write Group #3 of USB2 Port A access"]
pub type EnblWrGroup3ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUSB2PortAAccess` writer - Enable Write Group #3 of USB2 Port A access"]
pub type EnblWrGroup3ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUSB2PortAAccess` reader - Enable Write Group #4 of USB2 Port A access"]
pub type EnblWrGroup4ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUSB2PortAAccess` writer - Enable Write Group #4 of USB2 Port A access"]
pub type EnblWrGroup4ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUSB2PortAAccess` reader - Enable Write Group #5 of USB2 Port A access"]
pub type EnblWrGroup5ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUSB2PortAAccess` writer - Enable Write Group #5 of USB2 Port A access"]
pub type EnblWrGroup5ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1010PRIC1_010\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1010pric10100500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1010pric10100500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1010pric10100500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1010PRIC10100500` reader - Enable Reset Tolerance of PRIC1010PRIC1_010\\[05:00\\]"]
pub type EnblRstToleranceOfPric1010pric10100500R =
    crate::BitReader<EnblRstToleranceOfPric1010pric10100500>;
impl EnblRstToleranceOfPric1010pric10100500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1010pric10100500 {
        match self.bits {
            false => EnblRstToleranceOfPric1010pric10100500::ResetBySrst,
            true => EnblRstToleranceOfPric1010pric10100500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1010pric10100500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1010pric10100500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1010PRIC10100500` writer - Enable Reset Tolerance of PRIC1010PRIC1_010\\[05:00\\]"]
pub type EnblRstToleranceOfPric1010pric10100500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1010pric10100500>;
impl<'a, REG> EnblRstToleranceOfPric1010pric10100500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1010pric10100500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1010pric10100500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1010PRIC10100600` reader - Enable Write Protection of PRIC1010PRIC1_010\\[06:00\\]"]
pub type EnblWrProtOfPric1010pric10100600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1010PRIC10100600` writer - Enable Write Protection of PRIC1010PRIC1_010\\[06:00\\]"]
pub type EnblWrProtOfPric1010pric10100600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup0OfUSB2PortBAccess` reader - Enable Write Group #0 of USB2 Port B access"]
pub type EnblWrGroup0ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup0OfUSB2PortBAccess` writer - Enable Write Group #0 of USB2 Port B access"]
pub type EnblWrGroup0ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup1OfUSB2PortBAccess` reader - Enable Write Group #1 of USB2 Port B access"]
pub type EnblWrGroup1ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup1OfUSB2PortBAccess` writer - Enable Write Group #1 of USB2 Port B access"]
pub type EnblWrGroup1ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup2OfUSB2PortBAccess` reader - Enable Write Group #2 of USB2 Port B access"]
pub type EnblWrGroup2ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup2OfUSB2PortBAccess` writer - Enable Write Group #2 of USB2 Port B access"]
pub type EnblWrGroup2ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup3OfUSB2PortBAccess` reader - Enable Write Group #3 of USB2 Port B access"]
pub type EnblWrGroup3ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup3OfUSB2PortBAccess` writer - Enable Write Group #3 of USB2 Port B access"]
pub type EnblWrGroup3ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup4OfUSB2PortBAccess` reader - Enable Write Group #4 of USB2 Port B access"]
pub type EnblWrGroup4ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup4OfUSB2PortBAccess` writer - Enable Write Group #4 of USB2 Port B access"]
pub type EnblWrGroup4ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrGroup5OfUSB2PortBAccess` reader - Enable Write Group #5 of USB2 Port B access"]
pub type EnblWrGroup5ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblWrGroup5OfUSB2PortBAccess` writer - Enable Write Group #5 of USB2 Port B access"]
pub type EnblWrGroup5ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1010PRIC1_010\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1010pric10101308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1010pric10101308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1010pric10101308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1010PRIC10101308` reader - Enable Reset Tolerance of PRIC1010PRIC1_010\\[13:08\\]"]
pub type EnblRstToleranceOfPric1010pric10101308R =
    crate::BitReader<EnblRstToleranceOfPric1010pric10101308>;
impl EnblRstToleranceOfPric1010pric10101308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1010pric10101308 {
        match self.bits {
            false => EnblRstToleranceOfPric1010pric10101308::ResetBySrst,
            true => EnblRstToleranceOfPric1010pric10101308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1010pric10101308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1010pric10101308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1010PRIC10101308` writer - Enable Reset Tolerance of PRIC1010PRIC1_010\\[13:08\\]"]
pub type EnblRstToleranceOfPric1010pric10101308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1010pric10101308>;
impl<'a, REG> EnblRstToleranceOfPric1010pric10101308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1010pric10101308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1010pric10101308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1010PRIC10101408` reader - Enable Write Protection of PRIC1010PRIC1_010\\[14:08\\]"]
pub type EnblWrProtOfPric1010pric10101408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1010PRIC10101408` writer - Enable Write Protection of PRIC1010PRIC1_010\\[14:08\\]"]
pub type EnblWrProtOfPric1010pric10101408W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Enable Write Group #0 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_usb2port_aaccess(&self) -> EnblWrGroup0ofUsb2portAaccessR {
        EnblWrGroup0ofUsb2portAaccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_usb2port_aaccess(&self) -> EnblWrGroup1ofUsb2portAaccessR {
        EnblWrGroup1ofUsb2portAaccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_usb2port_aaccess(&self) -> EnblWrGroup2ofUsb2portAaccessR {
        EnblWrGroup2ofUsb2portAaccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_usb2port_aaccess(&self) -> EnblWrGroup3ofUsb2portAaccessR {
        EnblWrGroup3ofUsb2portAaccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_usb2port_aaccess(&self) -> EnblWrGroup4ofUsb2portAaccessR {
        EnblWrGroup4ofUsb2portAaccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_usb2port_aaccess(&self) -> EnblWrGroup5ofUsb2portAaccessR {
        EnblWrGroup5ofUsb2portAaccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1010PRIC1_010\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1010pric10100500(
        &self,
    ) -> EnblRstToleranceOfPric1010pric10100500R {
        EnblRstToleranceOfPric1010pric10100500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1010PRIC1_010\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1010pric10100600(&self) -> EnblWrProtOfPric1010pric10100600R {
        EnblWrProtOfPric1010pric10100600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_usb2port_baccess(&self) -> EnblWrGroup0ofUsb2portBaccessR {
        EnblWrGroup0ofUsb2portBaccessR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_usb2port_baccess(&self) -> EnblWrGroup1ofUsb2portBaccessR {
        EnblWrGroup1ofUsb2portBaccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_usb2port_baccess(&self) -> EnblWrGroup2ofUsb2portBaccessR {
        EnblWrGroup2ofUsb2portBaccessR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_usb2port_baccess(&self) -> EnblWrGroup3ofUsb2portBaccessR {
        EnblWrGroup3ofUsb2portBaccessR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_usb2port_baccess(&self) -> EnblWrGroup4ofUsb2portBaccessR {
        EnblWrGroup4ofUsb2portBaccessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_usb2port_baccess(&self) -> EnblWrGroup5ofUsb2portBaccessR {
        EnblWrGroup5ofUsb2portBaccessR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1010PRIC1_010\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1010pric10101308(
        &self,
    ) -> EnblRstToleranceOfPric1010pric10101308R {
        EnblRstToleranceOfPric1010pric10101308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1010PRIC1_010\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1010pric10101408(&self) -> EnblWrProtOfPric1010pric10101408R {
        EnblWrProtOfPric1010pric10101408R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Group #0 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_usb2port_aaccess(
        &mut self,
    ) -> EnblWrGroup0ofUsb2portAaccessW<PricIo010Spec> {
        EnblWrGroup0ofUsb2portAaccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Group #1 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_usb2port_aaccess(
        &mut self,
    ) -> EnblWrGroup1ofUsb2portAaccessW<PricIo010Spec> {
        EnblWrGroup1ofUsb2portAaccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Write Group #2 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_usb2port_aaccess(
        &mut self,
    ) -> EnblWrGroup2ofUsb2portAaccessW<PricIo010Spec> {
        EnblWrGroup2ofUsb2portAaccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Write Group #3 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_usb2port_aaccess(
        &mut self,
    ) -> EnblWrGroup3ofUsb2portAaccessW<PricIo010Spec> {
        EnblWrGroup3ofUsb2portAaccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Write Group #4 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_usb2port_aaccess(
        &mut self,
    ) -> EnblWrGroup4ofUsb2portAaccessW<PricIo010Spec> {
        EnblWrGroup4ofUsb2portAaccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Group #5 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_usb2port_aaccess(
        &mut self,
    ) -> EnblWrGroup5ofUsb2portAaccessW<PricIo010Spec> {
        EnblWrGroup5ofUsb2portAaccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1010PRIC1_010\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1010pric10100500(
        &mut self,
    ) -> EnblRstToleranceOfPric1010pric10100500W<PricIo010Spec> {
        EnblRstToleranceOfPric1010pric10100500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1010PRIC1_010\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1010pric10100600(
        &mut self,
    ) -> EnblWrProtOfPric1010pric10100600W<PricIo010Spec> {
        EnblWrProtOfPric1010pric10100600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Write Group #0 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group0of_usb2port_baccess(
        &mut self,
    ) -> EnblWrGroup0ofUsb2portBaccessW<PricIo010Spec> {
        EnblWrGroup0ofUsb2portBaccessW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Write Group #1 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group1of_usb2port_baccess(
        &mut self,
    ) -> EnblWrGroup1ofUsb2portBaccessW<PricIo010Spec> {
        EnblWrGroup1ofUsb2portBaccessW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Write Group #2 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group2of_usb2port_baccess(
        &mut self,
    ) -> EnblWrGroup2ofUsb2portBaccessW<PricIo010Spec> {
        EnblWrGroup2ofUsb2portBaccessW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Write Group #3 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group3of_usb2port_baccess(
        &mut self,
    ) -> EnblWrGroup3ofUsb2portBaccessW<PricIo010Spec> {
        EnblWrGroup3ofUsb2portBaccessW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Write Group #4 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group4of_usb2port_baccess(
        &mut self,
    ) -> EnblWrGroup4ofUsb2portBaccessW<PricIo010Spec> {
        EnblWrGroup4ofUsb2portBaccessW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Write Group #5 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_wr_group5of_usb2port_baccess(
        &mut self,
    ) -> EnblWrGroup5ofUsb2portBaccessW<PricIo010Spec> {
        EnblWrGroup5ofUsb2portBaccessW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1010PRIC1_010\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1010pric10101308(
        &mut self,
    ) -> EnblRstToleranceOfPric1010pric10101308W<PricIo010Spec> {
        EnblRstToleranceOfPric1010pric10101308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1010PRIC1_010\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1010pric10101408(
        &mut self,
    ) -> EnblWrProtOfPric1010pric10101408W<PricIo010Spec> {
        EnblWrProtOfPric1010pric10101408W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo010Spec> {
        Reserved3W::new(self, 16)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo010Spec> {
        Reserved2W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo010Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Master Write Group Setting Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo010Spec;
impl crate::RegisterSpec for PricIo010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io010::R`](R) reader structure"]
impl crate::Readable for PricIo010Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io010::W`](W) writer structure"]
impl crate::Writable for PricIo010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO010 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo010Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
