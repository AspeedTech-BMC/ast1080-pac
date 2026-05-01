#[doc = "Register `SCU3B8` reader"]
pub type R = crate::R<Scu3b8Spec>;
#[doc = "Register `SCU3B8` writer"]
pub type W = crate::W<Scu3b8Spec>;
#[doc = "Field `SCUUSBSEC10` reader - SCU_USB_SEC1_0"]
pub type Scuusbsec10R = crate::BitReader;
#[doc = "Field `SCUUSBSEC10` writer - SCU_USB_SEC1_0"]
pub type Scuusbsec10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUUSBSEC12` reader - SCU_USB_SEC1_2"]
pub type Scuusbsec12R = crate::BitReader;
#[doc = "Field `SCUUSBSEC12` writer - SCU_USB_SEC1_2"]
pub type Scuusbsec12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_USB_SEC1_0"]
    #[inline(always)]
    pub fn scuusbsec10(&self) -> Scuusbsec10R {
        Scuusbsec10R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_USB_SEC1_2"]
    #[inline(always)]
    pub fn scuusbsec12(&self) -> Scuusbsec12R {
        Scuusbsec12R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_USB_SEC1_0"]
    #[inline(always)]
    pub fn scuusbsec10(&mut self) -> Scuusbsec10W<Scu3b8Spec> {
        Scuusbsec10W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_USB_SEC1_2"]
    #[inline(always)]
    pub fn scuusbsec12(&mut self) -> Scuusbsec12W<Scu3b8Spec> {
        Scuusbsec12W::new(self, 2)
    }
}
#[doc = "USB Controler Secure Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu3b8Spec;
impl crate::RegisterSpec for Scu3b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu3b8::R`](R) reader structure"]
impl crate::Readable for Scu3b8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu3b8::W`](W) writer structure"]
impl crate::Writable for Scu3b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU3B8 to value 0"]
impl crate::Resettable for Scu3b8Spec {}
