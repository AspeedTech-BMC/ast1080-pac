#[doc = "Register `SCUD14` reader"]
pub type R = crate::R<Scud14Spec>;
#[doc = "Register `SCUD14` writer"]
pub type W = crate::W<Scud14Spec>;
#[doc = "Field `SCUREGSEC32F0` reader - SCU_REG_SEC3_2F0"]
pub type Scuregsec32f0R = crate::BitReader;
#[doc = "Field `SCUREGSEC32F0` writer - SCU_REG_SEC3_2F0"]
pub type Scuregsec32f0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC32F4` reader - SCU_REG_SEC3_2F4"]
pub type Scuregsec32f4R = crate::BitReader;
#[doc = "Field `SCUREGSEC32F4` writer - SCU_REG_SEC3_2F4"]
pub type Scuregsec32f4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC32F8` reader - SCU_REG_SEC3_2F8"]
pub type Scuregsec32f8R = crate::BitReader;
#[doc = "Field `SCUREGSEC32F8` writer - SCU_REG_SEC3_2F8"]
pub type Scuregsec32f8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - SCU_REG_SEC3_2F0"]
    #[inline(always)]
    pub fn scuregsec32f0(&self) -> Scuregsec32f0R {
        Scuregsec32f0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCU_REG_SEC3_2F4"]
    #[inline(always)]
    pub fn scuregsec32f4(&self) -> Scuregsec32f4R {
        Scuregsec32f4R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SCU_REG_SEC3_2F8"]
    #[inline(always)]
    pub fn scuregsec32f8(&self) -> Scuregsec32f8R {
        Scuregsec32f8R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - SCU_REG_SEC3_2F0"]
    #[inline(always)]
    pub fn scuregsec32f0(&mut self) -> Scuregsec32f0W<Scud14Spec> {
        Scuregsec32f0W::new(self, 28)
    }
    #[doc = "Bit 29 - SCU_REG_SEC3_2F4"]
    #[inline(always)]
    pub fn scuregsec32f4(&mut self) -> Scuregsec32f4W<Scud14Spec> {
        Scuregsec32f4W::new(self, 29)
    }
    #[doc = "Bit 30 - SCU_REG_SEC3_2F8"]
    #[inline(always)]
    pub fn scuregsec32f8(&mut self) -> Scuregsec32f8W<Scud14Spec> {
        Scuregsec32f8W::new(self, 30)
    }
}
#[doc = "Secure3 Control 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scud14Spec;
impl crate::RegisterSpec for Scud14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scud14::R`](R) reader structure"]
impl crate::Readable for Scud14Spec {}
#[doc = "`write(|w| ..)` method takes [`scud14::W`](W) writer structure"]
impl crate::Writable for Scud14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUD14 to value 0"]
impl crate::Resettable for Scud14Spec {}
