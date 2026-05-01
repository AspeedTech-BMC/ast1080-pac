#[doc = "Register `HUB3C` reader"]
pub type R = crate::R<Hub3cSpec>;
#[doc = "Register `HUB3C` writer"]
pub type W = crate::W<Hub3cSpec>;
#[doc = "Field `HubPortStatusChangeBit` reader - Hub Port Status Change Bit"]
pub type HubPortStatusChangeBitR = crate::BitReader;
#[doc = "Field `HubPortStatusChangeBit` writer - Hub Port Status Change Bit"]
pub type HubPortStatusChangeBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Port1StatusChangeBitDev1` reader - Port #1 Status Change Bit (Device #1)"]
pub type Port1statusChangeBitDev1R = crate::BitReader;
#[doc = "Field `Port1StatusChangeBitDev1` writer - Port #1 Status Change Bit (Device #1)"]
pub type Port1statusChangeBitDev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Port2StatusChangeBitDev2` reader - Port #2 Status Change Bit (Device #2)"]
pub type Port2statusChangeBitDev2R = crate::BitReader;
#[doc = "Field `Port2StatusChangeBitDev2` writer - Port #2 Status Change Bit (Device #2)"]
pub type Port2statusChangeBitDev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Port3StatusChangeBitDev3` reader - Port #3 Status Change Bit (Device #3)"]
pub type Port3statusChangeBitDev3R = crate::BitReader;
#[doc = "Field `Port3StatusChangeBitDev3` writer - Port #3 Status Change Bit (Device #3)"]
pub type Port3statusChangeBitDev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Port4StatusChangeBitDev4` reader - Port #4 Status Change Bit (Device #4)"]
pub type Port4statusChangeBitDev4R = crate::BitReader;
#[doc = "Field `Port4StatusChangeBitDev4` writer - Port #4 Status Change Bit (Device #4)"]
pub type Port4statusChangeBitDev4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Port5StatusChangeBitDev5` reader - Port #5 Status Change Bit (Device #5)"]
pub type Port5statusChangeBitDev5R = crate::BitReader;
#[doc = "Field `Port5StatusChangeBitDev5` writer - Port #5 Status Change Bit (Device #5)"]
pub type Port5statusChangeBitDev5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Port6StatusChangeBitDev6` reader - Port #6 Status Change Bit (Device #6)"]
pub type Port6statusChangeBitDev6R = crate::BitReader;
#[doc = "Field `Port6StatusChangeBitDev6` writer - Port #6 Status Change Bit (Device #6)"]
pub type Port6statusChangeBitDev6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Port7StatusChangeBitDev7` reader - Port #7 Status Change Bit (Device #7)"]
pub type Port7statusChangeBitDev7R = crate::BitReader;
#[doc = "Field `Port7StatusChangeBitDev7` writer - Port #7 Status Change Bit (Device #7)"]
pub type Port7statusChangeBitDev7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hub Port Status Change Bit"]
    #[inline(always)]
    pub fn hub_port_status_change_bit(&self) -> HubPortStatusChangeBitR {
        HubPortStatusChangeBitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port #1 Status Change Bit (Device #1)"]
    #[inline(always)]
    pub fn port1status_change_bit_dev1(&self) -> Port1statusChangeBitDev1R {
        Port1statusChangeBitDev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port #2 Status Change Bit (Device #2)"]
    #[inline(always)]
    pub fn port2status_change_bit_dev2(&self) -> Port2statusChangeBitDev2R {
        Port2statusChangeBitDev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port #3 Status Change Bit (Device #3)"]
    #[inline(always)]
    pub fn port3status_change_bit_dev3(&self) -> Port3statusChangeBitDev3R {
        Port3statusChangeBitDev3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port #4 Status Change Bit (Device #4)"]
    #[inline(always)]
    pub fn port4status_change_bit_dev4(&self) -> Port4statusChangeBitDev4R {
        Port4statusChangeBitDev4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port #5 Status Change Bit (Device #5)"]
    #[inline(always)]
    pub fn port5status_change_bit_dev5(&self) -> Port5statusChangeBitDev5R {
        Port5statusChangeBitDev5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port #6 Status Change Bit (Device #6)"]
    #[inline(always)]
    pub fn port6status_change_bit_dev6(&self) -> Port6statusChangeBitDev6R {
        Port6statusChangeBitDev6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port #7 Status Change Bit (Device #7)"]
    #[inline(always)]
    pub fn port7status_change_bit_dev7(&self) -> Port7statusChangeBitDev7R {
        Port7statusChangeBitDev7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hub Port Status Change Bit"]
    #[inline(always)]
    pub fn hub_port_status_change_bit(&mut self) -> HubPortStatusChangeBitW<Hub3cSpec> {
        HubPortStatusChangeBitW::new(self, 0)
    }
    #[doc = "Bit 1 - Port #1 Status Change Bit (Device #1)"]
    #[inline(always)]
    pub fn port1status_change_bit_dev1(&mut self) -> Port1statusChangeBitDev1W<Hub3cSpec> {
        Port1statusChangeBitDev1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port #2 Status Change Bit (Device #2)"]
    #[inline(always)]
    pub fn port2status_change_bit_dev2(&mut self) -> Port2statusChangeBitDev2W<Hub3cSpec> {
        Port2statusChangeBitDev2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port #3 Status Change Bit (Device #3)"]
    #[inline(always)]
    pub fn port3status_change_bit_dev3(&mut self) -> Port3statusChangeBitDev3W<Hub3cSpec> {
        Port3statusChangeBitDev3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port #4 Status Change Bit (Device #4)"]
    #[inline(always)]
    pub fn port4status_change_bit_dev4(&mut self) -> Port4statusChangeBitDev4W<Hub3cSpec> {
        Port4statusChangeBitDev4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port #5 Status Change Bit (Device #5)"]
    #[inline(always)]
    pub fn port5status_change_bit_dev5(&mut self) -> Port5statusChangeBitDev5W<Hub3cSpec> {
        Port5statusChangeBitDev5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port #6 Status Change Bit (Device #6)"]
    #[inline(always)]
    pub fn port6status_change_bit_dev6(&mut self) -> Port6statusChangeBitDev6W<Hub3cSpec> {
        Port6statusChangeBitDev6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port #7 Status Change Bit (Device #7)"]
    #[inline(always)]
    pub fn port7status_change_bit_dev7(&mut self) -> Port7statusChangeBitDev7W<Hub3cSpec> {
        Port7statusChangeBitDev7W::new(self, 7)
    }
}
#[doc = "Endpoint 1 Status Change Bitmap Data\n\nYou can [`read`](crate::Reg::read) this register and get [`hub3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub3cSpec;
impl crate::RegisterSpec for Hub3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub3c::R`](R) reader structure"]
impl crate::Readable for Hub3cSpec {}
#[doc = "`write(|w| ..)` method takes [`hub3c::W`](W) writer structure"]
impl crate::Writable for Hub3cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB3C to value 0"]
impl crate::Resettable for Hub3cSpec {}
