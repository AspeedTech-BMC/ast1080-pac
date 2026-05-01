#[doc = "Register `SGPIO020` reader"]
pub type R = crate::R<Sgpio020Spec>;
#[doc = "Register `SGPIO020` writer"]
pub type W = crate::W<Sgpio020Spec>;
#[doc = "Field `EnblSGPIOBit07As80hOutput` reader - Enable SGPIO bit 0~7 as 80h output"]
pub type EnblSgpiobit07as80hOutputR = crate::BitReader;
#[doc = "Field `EnblSGPIOBit07As80hOutput` writer - Enable SGPIO bit 0~7 as 80h output"]
pub type EnblSgpiobit07as80hOutputW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSGPIOBit815As80hOutput` reader - Enable SGPIO bit 8~15 as 80h output"]
pub type EnblSgpiobit815as80hOutputR = crate::BitReader;
#[doc = "Field `EnblSGPIOBit815As80hOutput` writer - Enable SGPIO bit 8~15 as 80h output"]
pub type EnblSgpiobit815as80hOutputW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable SGPIO bit 0~7 as 80h output"]
    #[inline(always)]
    pub fn enbl_sgpiobit07as80h_output(&self) -> EnblSgpiobit07as80hOutputR {
        EnblSgpiobit07as80hOutputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable SGPIO bit 8~15 as 80h output"]
    #[inline(always)]
    pub fn enbl_sgpiobit815as80h_output(&self) -> EnblSgpiobit815as80hOutputR {
        EnblSgpiobit815as80hOutputR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SGPIO bit 0~7 as 80h output"]
    #[inline(always)]
    pub fn enbl_sgpiobit07as80h_output(&mut self) -> EnblSgpiobit07as80hOutputW<Sgpio020Spec> {
        EnblSgpiobit07as80hOutputW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable SGPIO bit 8~15 as 80h output"]
    #[inline(always)]
    pub fn enbl_sgpiobit815as80h_output(&mut self) -> EnblSgpiobit815as80hOutputW<Sgpio020Spec> {
        EnblSgpiobit815as80hOutputW::new(self, 1)
    }
}
#[doc = "80h Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio020Spec;
impl crate::RegisterSpec for Sgpio020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio020::R`](R) reader structure"]
impl crate::Readable for Sgpio020Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio020::W`](W) writer structure"]
impl crate::Writable for Sgpio020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO020 to value 0"]
impl crate::Resettable for Sgpio020Spec {}
