#[doc = "Register `HUB38` reader"]
pub type R = crate::R<Hub38Spec>;
#[doc = "Register `HUB38` writer"]
pub type W = crate::W<Hub38Spec>;
#[doc = "Field `EnblEndpoint1` reader - Enable Endpoint 1"]
pub type EnblEndpoint1R = crate::BitReader;
#[doc = "Field `EnblEndpoint1` writer - Enable Endpoint 1"]
pub type EnblEndpoint1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Endpoint1STALLCtrl` reader - Endpoint 1 STALL control"]
pub type Endpoint1stallctrlR = crate::BitReader;
#[doc = "Field `Endpoint1STALLCtrl` writer - Endpoint 1 STALL control"]
pub type Endpoint1stallctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstEndpoint1DataToggleBitToDATA0` reader - Reset Endpoint 1 data toggle bit to DATA0"]
pub type RstEndpoint1dataToggleBitToData0R = crate::BitReader;
#[doc = "Field `RstEndpoint1DataToggleBitToDATA0` writer - Reset Endpoint 1 data toggle bit to DATA0"]
pub type RstEndpoint1dataToggleBitToData0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Enable Endpoint 1"]
    #[inline(always)]
    pub fn enbl_endpoint1(&self) -> EnblEndpoint1R {
        EnblEndpoint1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 STALL control"]
    #[inline(always)]
    pub fn endpoint1stallctrl(&self) -> Endpoint1stallctrlR {
        Endpoint1stallctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset Endpoint 1 data toggle bit to DATA0"]
    #[inline(always)]
    pub fn rst_endpoint1data_toggle_bit_to_data0(&self) -> RstEndpoint1dataToggleBitToData0R {
        RstEndpoint1dataToggleBitToData0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Endpoint 1"]
    #[inline(always)]
    pub fn enbl_endpoint1(&mut self) -> EnblEndpoint1W<Hub38Spec> {
        EnblEndpoint1W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint 1 STALL control"]
    #[inline(always)]
    pub fn endpoint1stallctrl(&mut self) -> Endpoint1stallctrlW<Hub38Spec> {
        Endpoint1stallctrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset Endpoint 1 data toggle bit to DATA0"]
    #[inline(always)]
    pub fn rst_endpoint1data_toggle_bit_to_data0(
        &mut self,
    ) -> RstEndpoint1dataToggleBitToData0W<Hub38Spec> {
        RstEndpoint1dataToggleBitToData0W::new(self, 2)
    }
}
#[doc = "Endpoint 1 Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub38Spec;
impl crate::RegisterSpec for Hub38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub38::R`](R) reader structure"]
impl crate::Readable for Hub38Spec {}
#[doc = "`write(|w| ..)` method takes [`hub38::W`](W) writer structure"]
impl crate::Writable for Hub38Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB38 to value 0"]
impl crate::Resettable for Hub38Spec {}
