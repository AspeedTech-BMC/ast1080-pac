#[doc = "Register `PRIC_IO110` reader"]
pub type R = crate::R<PricIo110Spec>;
#[doc = "Register `PRIC_IO110` writer"]
pub type W = crate::W<PricIo110Spec>;
#[doc = "Field `EnblReadGroup0OfUSB2PortAAccess` reader - Enable Read Group #0 of USB2 Port A access"]
pub type EnblReadGroup0ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUSB2PortAAccess` writer - Enable Read Group #0 of USB2 Port A access"]
pub type EnblReadGroup0ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUSB2PortAAccess` reader - Enable Read Group #1 of USB2 Port A access"]
pub type EnblReadGroup1ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUSB2PortAAccess` writer - Enable Read Group #1 of USB2 Port A access"]
pub type EnblReadGroup1ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUSB2PortAAccess` reader - Enable Read Group #2 of USB2 Port A access"]
pub type EnblReadGroup2ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUSB2PortAAccess` writer - Enable Read Group #2 of USB2 Port A access"]
pub type EnblReadGroup2ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUSB2PortAAccess` reader - Enable Read Group #3 of USB2 Port A access"]
pub type EnblReadGroup3ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUSB2PortAAccess` writer - Enable Read Group #3 of USB2 Port A access"]
pub type EnblReadGroup3ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUSB2PortAAccess` reader - Enable Read Group #4 of USB2 Port A access"]
pub type EnblReadGroup4ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUSB2PortAAccess` writer - Enable Read Group #4 of USB2 Port A access"]
pub type EnblReadGroup4ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUSB2PortAAccess` reader - Enable Read Group #5 of USB2 Port A access"]
pub type EnblReadGroup5ofUsb2portAaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUSB2PortAAccess` writer - Enable Read Group #5 of USB2 Port A access"]
pub type EnblReadGroup5ofUsb2portAaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1110PRIC1_110\\[05:00\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1110pric11100500 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1110pric11100500> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1110pric11100500) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1110PRIC11100500` reader - Enable Reset Tolerance of PRIC1110PRIC1_110\\[05:00\\]"]
pub type EnblRstToleranceOfPric1110pric11100500R =
    crate::BitReader<EnblRstToleranceOfPric1110pric11100500>;
impl EnblRstToleranceOfPric1110pric11100500R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1110pric11100500 {
        match self.bits {
            false => EnblRstToleranceOfPric1110pric11100500::ResetBySrst,
            true => EnblRstToleranceOfPric1110pric11100500::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1110pric11100500::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1110pric11100500::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1110PRIC11100500` writer - Enable Reset Tolerance of PRIC1110PRIC1_110\\[05:00\\]"]
pub type EnblRstToleranceOfPric1110pric11100500W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1110pric11100500>;
impl<'a, REG> EnblRstToleranceOfPric1110pric11100500W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1110pric11100500::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1110pric11100500::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1110PRIC11100600` reader - Enable Write Protection of PRIC1110PRIC1_110\\[06:00\\]"]
pub type EnblWrProtOfPric1110pric11100600R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1110PRIC11100600` writer - Enable Write Protection of PRIC1110PRIC1_110\\[06:00\\]"]
pub type EnblWrProtOfPric1110pric11100600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup0OfUSB2PortBAccess` reader - Enable Read Group #0 of USB2 Port B access"]
pub type EnblReadGroup0ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup0OfUSB2PortBAccess` writer - Enable Read Group #0 of USB2 Port B access"]
pub type EnblReadGroup0ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup1OfUSB2PortBAccess` reader - Enable Read Group #1 of USB2 Port B access"]
pub type EnblReadGroup1ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup1OfUSB2PortBAccess` writer - Enable Read Group #1 of USB2 Port B access"]
pub type EnblReadGroup1ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup2OfUSB2PortBAccess` reader - Enable Read Group #2 of USB2 Port B access"]
pub type EnblReadGroup2ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup2OfUSB2PortBAccess` writer - Enable Read Group #2 of USB2 Port B access"]
pub type EnblReadGroup2ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup3OfUSB2PortBAccess` reader - Enable Read Group #3 of USB2 Port B access"]
pub type EnblReadGroup3ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup3OfUSB2PortBAccess` writer - Enable Read Group #3 of USB2 Port B access"]
pub type EnblReadGroup3ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup4OfUSB2PortBAccess` reader - Enable Read Group #4 of USB2 Port B access"]
pub type EnblReadGroup4ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup4OfUSB2PortBAccess` writer - Enable Read Group #4 of USB2 Port B access"]
pub type EnblReadGroup4ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblReadGroup5OfUSB2PortBAccess` reader - Enable Read Group #5 of USB2 Port B access"]
pub type EnblReadGroup5ofUsb2portBaccessR = crate::BitReader;
#[doc = "Field `EnblReadGroup5OfUSB2PortBAccess` writer - Enable Read Group #5 of USB2 Port B access"]
pub type EnblReadGroup5ofUsb2portBaccessW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enable Reset Tolerance of PRIC1110PRIC1_110\\[13:08\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblRstToleranceOfPric1110pric11101308 {
    #[doc = "0: Reset by SRST\\#"]
    ResetBySrst = 0,
    #[doc = "1: Reset by SRST\\# and PSP reset"]
    ResetBySrstAndPspReset = 1,
}
impl From<EnblRstToleranceOfPric1110pric11101308> for bool {
    #[inline(always)]
    fn from(variant: EnblRstToleranceOfPric1110pric11101308) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1110PRIC11101308` reader - Enable Reset Tolerance of PRIC1110PRIC1_110\\[13:08\\]"]
pub type EnblRstToleranceOfPric1110pric11101308R =
    crate::BitReader<EnblRstToleranceOfPric1110pric11101308>;
impl EnblRstToleranceOfPric1110pric11101308R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblRstToleranceOfPric1110pric11101308 {
        match self.bits {
            false => EnblRstToleranceOfPric1110pric11101308::ResetBySrst,
            true => EnblRstToleranceOfPric1110pric11101308::ResetBySrstAndPspReset,
        }
    }
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn is_reset_by_srst(&self) -> bool {
        *self == EnblRstToleranceOfPric1110pric11101308::ResetBySrst
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn is_reset_by_srst_and_psp_reset(&self) -> bool {
        *self == EnblRstToleranceOfPric1110pric11101308::ResetBySrstAndPspReset
    }
}
#[doc = "Field `EnblRstToleranceOfPRIC1110PRIC11101308` writer - Enable Reset Tolerance of PRIC1110PRIC1_110\\[13:08\\]"]
pub type EnblRstToleranceOfPric1110pric11101308W<'a, REG> =
    crate::BitWriter<'a, REG, EnblRstToleranceOfPric1110pric11101308>;
impl<'a, REG> EnblRstToleranceOfPric1110pric11101308W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset by SRST\\#"]
    #[inline(always)]
    pub fn reset_by_srst(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1110pric11101308::ResetBySrst)
    }
    #[doc = "Reset by SRST\\# and PSP reset"]
    #[inline(always)]
    pub fn reset_by_srst_and_psp_reset(self) -> &'a mut crate::W<REG> {
        self.variant(EnblRstToleranceOfPric1110pric11101308::ResetBySrstAndPspReset)
    }
}
#[doc = "Field `EnblWrProtOfPRIC1110PRIC11101408` reader - Enable Write Protection of PRIC1110PRIC1_110\\[14:08\\]"]
pub type EnblWrProtOfPric1110pric11101408R = crate::BitReader;
#[doc = "Field `EnblWrProtOfPRIC1110PRIC11101408` writer - Enable Write Protection of PRIC1110PRIC1_110\\[14:08\\]"]
pub type EnblWrProtOfPric1110pric11101408W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - Enable Read Group #0 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group0of_usb2port_aaccess(&self) -> EnblReadGroup0ofUsb2portAaccessR {
        EnblReadGroup0ofUsb2portAaccessR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group1of_usb2port_aaccess(&self) -> EnblReadGroup1ofUsb2portAaccessR {
        EnblReadGroup1ofUsb2portAaccessR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group2of_usb2port_aaccess(&self) -> EnblReadGroup2ofUsb2portAaccessR {
        EnblReadGroup2ofUsb2portAaccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group3of_usb2port_aaccess(&self) -> EnblReadGroup3ofUsb2portAaccessR {
        EnblReadGroup3ofUsb2portAaccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group4of_usb2port_aaccess(&self) -> EnblReadGroup4ofUsb2portAaccessR {
        EnblReadGroup4ofUsb2portAaccessR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group5of_usb2port_aaccess(&self) -> EnblReadGroup5ofUsb2portAaccessR {
        EnblReadGroup5ofUsb2portAaccessR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1110PRIC1_110\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1110pric11100500(
        &self,
    ) -> EnblRstToleranceOfPric1110pric11100500R {
        EnblRstToleranceOfPric1110pric11100500R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1110PRIC1_110\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1110pric11100600(&self) -> EnblWrProtOfPric1110pric11100600R {
        EnblWrProtOfPric1110pric11100600R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group0of_usb2port_baccess(&self) -> EnblReadGroup0ofUsb2portBaccessR {
        EnblReadGroup0ofUsb2portBaccessR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group1of_usb2port_baccess(&self) -> EnblReadGroup1ofUsb2portBaccessR {
        EnblReadGroup1ofUsb2portBaccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group2of_usb2port_baccess(&self) -> EnblReadGroup2ofUsb2portBaccessR {
        EnblReadGroup2ofUsb2portBaccessR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group3of_usb2port_baccess(&self) -> EnblReadGroup3ofUsb2portBaccessR {
        EnblReadGroup3ofUsb2portBaccessR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group4of_usb2port_baccess(&self) -> EnblReadGroup4ofUsb2portBaccessR {
        EnblReadGroup4ofUsb2portBaccessR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group5of_usb2port_baccess(&self) -> EnblReadGroup5ofUsb2portBaccessR {
        EnblReadGroup5ofUsb2portBaccessR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1110PRIC1_110\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1110pric11101308(
        &self,
    ) -> EnblRstToleranceOfPric1110pric11101308R {
        EnblRstToleranceOfPric1110pric11101308R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1110PRIC1_110\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1110pric11101408(&self) -> EnblWrProtOfPric1110pric11101408R {
        EnblWrProtOfPric1110pric11101408R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 0 - Enable Read Group #0 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group0of_usb2port_aaccess(
        &mut self,
    ) -> EnblReadGroup0ofUsb2portAaccessW<PricIo110Spec> {
        EnblReadGroup0ofUsb2portAaccessW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Read Group #1 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group1of_usb2port_aaccess(
        &mut self,
    ) -> EnblReadGroup1ofUsb2portAaccessW<PricIo110Spec> {
        EnblReadGroup1ofUsb2portAaccessW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Read Group #2 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group2of_usb2port_aaccess(
        &mut self,
    ) -> EnblReadGroup2ofUsb2portAaccessW<PricIo110Spec> {
        EnblReadGroup2ofUsb2portAaccessW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Read Group #3 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group3of_usb2port_aaccess(
        &mut self,
    ) -> EnblReadGroup3ofUsb2portAaccessW<PricIo110Spec> {
        EnblReadGroup3ofUsb2portAaccessW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Read Group #4 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group4of_usb2port_aaccess(
        &mut self,
    ) -> EnblReadGroup4ofUsb2portAaccessW<PricIo110Spec> {
        EnblReadGroup4ofUsb2portAaccessW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Read Group #5 of USB2 Port A access"]
    #[inline(always)]
    pub fn enbl_read_group5of_usb2port_aaccess(
        &mut self,
    ) -> EnblReadGroup5ofUsb2portAaccessW<PricIo110Spec> {
        EnblReadGroup5ofUsb2portAaccessW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Reset Tolerance of PRIC1110PRIC1_110\\[05:00\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1110pric11100500(
        &mut self,
    ) -> EnblRstToleranceOfPric1110pric11100500W<PricIo110Spec> {
        EnblRstToleranceOfPric1110pric11100500W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of PRIC1110PRIC1_110\\[06:00\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1110pric11100600(
        &mut self,
    ) -> EnblWrProtOfPric1110pric11100600W<PricIo110Spec> {
        EnblWrProtOfPric1110pric11100600W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Read Group #0 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group0of_usb2port_baccess(
        &mut self,
    ) -> EnblReadGroup0ofUsb2portBaccessW<PricIo110Spec> {
        EnblReadGroup0ofUsb2portBaccessW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Read Group #1 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group1of_usb2port_baccess(
        &mut self,
    ) -> EnblReadGroup1ofUsb2portBaccessW<PricIo110Spec> {
        EnblReadGroup1ofUsb2portBaccessW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Read Group #2 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group2of_usb2port_baccess(
        &mut self,
    ) -> EnblReadGroup2ofUsb2portBaccessW<PricIo110Spec> {
        EnblReadGroup2ofUsb2portBaccessW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Read Group #3 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group3of_usb2port_baccess(
        &mut self,
    ) -> EnblReadGroup3ofUsb2portBaccessW<PricIo110Spec> {
        EnblReadGroup3ofUsb2portBaccessW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Read Group #4 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group4of_usb2port_baccess(
        &mut self,
    ) -> EnblReadGroup4ofUsb2portBaccessW<PricIo110Spec> {
        EnblReadGroup4ofUsb2portBaccessW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Read Group #5 of USB2 Port B access"]
    #[inline(always)]
    pub fn enbl_read_group5of_usb2port_baccess(
        &mut self,
    ) -> EnblReadGroup5ofUsb2portBaccessW<PricIo110Spec> {
        EnblReadGroup5ofUsb2portBaccessW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Reset Tolerance of PRIC1110PRIC1_110\\[13:08\\]"]
    #[inline(always)]
    pub fn enbl_rst_tolerance_of_pric1110pric11101308(
        &mut self,
    ) -> EnblRstToleranceOfPric1110pric11101308W<PricIo110Spec> {
        EnblRstToleranceOfPric1110pric11101308W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Write Protection of PRIC1110PRIC1_110\\[14:08\\]"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_pric1110pric11101408(
        &mut self,
    ) -> EnblWrProtOfPric1110pric11101408W<PricIo110Spec> {
        EnblWrProtOfPric1110pric11101408W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo110Spec> {
        Reserved3W::new(self, 16)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo110Spec> {
        Reserved2W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo110Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Master Read Group Setting Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo110Spec;
impl crate::RegisterSpec for PricIo110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io110::R`](R) reader structure"]
impl crate::Readable for PricIo110Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io110::W`](W) writer structure"]
impl crate::Writable for PricIo110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO110 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo110Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
