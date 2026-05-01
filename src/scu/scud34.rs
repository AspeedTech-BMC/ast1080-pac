#[doc = "Register `SCUD34` reader"]
pub type R = crate::R<Scud34Spec>;
#[doc = "Register `SCUD34` writer"]
pub type W = crate::W<Scud34Spec>;
#[doc = "Field `SCUREGSEC3680` reader - SCU_REG_SEC3_680"]
pub type Scuregsec3680R = crate::BitReader;
#[doc = "Field `SCUREGSEC3680` writer - SCU_REG_SEC3_680"]
pub type Scuregsec3680W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3684` reader - SCU_REG_SEC3_684"]
pub type Scuregsec3684R = crate::BitReader;
#[doc = "Field `SCUREGSEC3684` writer - SCU_REG_SEC3_684"]
pub type Scuregsec3684W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3688` reader - SCU_REG_SEC3_688"]
pub type Scuregsec3688R = crate::BitReader;
#[doc = "Field `SCUREGSEC3688` writer - SCU_REG_SEC3_688"]
pub type Scuregsec3688W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC368C` reader - SCU_REG_SEC3_68C"]
pub type Scuregsec368cR = crate::BitReader;
#[doc = "Field `SCUREGSEC368C` writer - SCU_REG_SEC3_68C"]
pub type Scuregsec368cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC36FC` reader - SCU_REG_SEC3_6FC"]
pub type Scuregsec36fcR = crate::BitReader;
#[doc = "Field `SCUREGSEC36FC` writer - SCU_REG_SEC3_6FC"]
pub type Scuregsec36fcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC3_680"]
    #[inline(always)]
    pub fn scuregsec3680(&self) -> Scuregsec3680R {
        Scuregsec3680R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC3_684"]
    #[inline(always)]
    pub fn scuregsec3684(&self) -> Scuregsec3684R {
        Scuregsec3684R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC3_688"]
    #[inline(always)]
    pub fn scuregsec3688(&self) -> Scuregsec3688R {
        Scuregsec3688R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_SEC3_68C"]
    #[inline(always)]
    pub fn scuregsec368c(&self) -> Scuregsec368cR {
        Scuregsec368cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_REG_SEC3_6FC"]
    #[inline(always)]
    pub fn scuregsec36fc(&self) -> Scuregsec36fcR {
        Scuregsec36fcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC3_680"]
    #[inline(always)]
    pub fn scuregsec3680(&mut self) -> Scuregsec3680W<Scud34Spec> {
        Scuregsec3680W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC3_684"]
    #[inline(always)]
    pub fn scuregsec3684(&mut self) -> Scuregsec3684W<Scud34Spec> {
        Scuregsec3684W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_SEC3_688"]
    #[inline(always)]
    pub fn scuregsec3688(&mut self) -> Scuregsec3688W<Scud34Spec> {
        Scuregsec3688W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_SEC3_68C"]
    #[inline(always)]
    pub fn scuregsec368c(&mut self) -> Scuregsec368cW<Scud34Spec> {
        Scuregsec368cW::new(self, 3)
    }
    #[doc = "Bit 31 - SCU_REG_SEC3_6FC"]
    #[inline(always)]
    pub fn scuregsec36fc(&mut self) -> Scuregsec36fcW<Scud34Spec> {
        Scuregsec36fcW::new(self, 31)
    }
}
#[doc = "Secure3 Control 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scud34Spec;
impl crate::RegisterSpec for Scud34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scud34::R`](R) reader structure"]
impl crate::Readable for Scud34Spec {}
#[doc = "`write(|w| ..)` method takes [`scud34::W`](W) writer structure"]
impl crate::Writable for Scud34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUD34 to value 0"]
impl crate::Resettable for Scud34Spec {}
