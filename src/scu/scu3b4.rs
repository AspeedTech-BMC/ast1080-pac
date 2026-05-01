#[doc = "Register `SCU3B4` reader"]
pub type R = crate::R<Scu3b4Spec>;
#[doc = "Register `SCU3B4` writer"]
pub type W = crate::W<Scu3b4Spec>;
#[doc = "Field `SCUUSBLOCK0` reader - SCU_USB_LOCK_0"]
pub type Scuusblock0R = crate::BitReader;
#[doc = "Field `SCUUSBLOCK0` writer - SCU_USB_LOCK_0"]
pub type Scuusblock0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUUSBLOCK2` reader - SCU_USB_LOCK_2"]
pub type Scuusblock2R = crate::BitReader;
#[doc = "Field `SCUUSBLOCK2` writer - SCU_USB_LOCK_2"]
pub type Scuusblock2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_USB_LOCK_0"]
    #[inline(always)]
    pub fn scuusblock0(&self) -> Scuusblock0R {
        Scuusblock0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_USB_LOCK_2"]
    #[inline(always)]
    pub fn scuusblock2(&self) -> Scuusblock2R {
        Scuusblock2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_USB_LOCK_0"]
    #[inline(always)]
    pub fn scuusblock0(&mut self) -> Scuusblock0W<Scu3b4Spec> {
        Scuusblock0W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_USB_LOCK_2"]
    #[inline(always)]
    pub fn scuusblock2(&mut self) -> Scuusblock2W<Scu3b4Spec> {
        Scuusblock2W::new(self, 2)
    }
}
#[doc = "USB Controler Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu3b4Spec;
impl crate::RegisterSpec for Scu3b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu3b4::R`](R) reader structure"]
impl crate::Readable for Scu3b4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu3b4::W`](W) writer structure"]
impl crate::Writable for Scu3b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU3B4 to value 0"]
impl crate::Resettable for Scu3b4Spec {}
