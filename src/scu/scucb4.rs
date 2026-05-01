#[doc = "Register `SCUCB4` reader"]
pub type R = crate::R<Scucb4Spec>;
#[doc = "Register `SCUCB4` writer"]
pub type W = crate::W<Scucb4Spec>;
#[doc = "Field `SCUREGSEC2680` reader - SCU_REG_SEC2_680"]
pub type Scuregsec2680R = crate::BitReader;
#[doc = "Field `SCUREGSEC2680` writer - SCU_REG_SEC2_680"]
pub type Scuregsec2680W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2684` reader - SCU_REG_SEC2_684"]
pub type Scuregsec2684R = crate::BitReader;
#[doc = "Field `SCUREGSEC2684` writer - SCU_REG_SEC2_684"]
pub type Scuregsec2684W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2688` reader - SCU_REG_SEC2_688"]
pub type Scuregsec2688R = crate::BitReader;
#[doc = "Field `SCUREGSEC2688` writer - SCU_REG_SEC2_688"]
pub type Scuregsec2688W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC268C` reader - SCU_REG_SEC2_68C"]
pub type Scuregsec268cR = crate::BitReader;
#[doc = "Field `SCUREGSEC268C` writer - SCU_REG_SEC2_68C"]
pub type Scuregsec268cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC26FC` reader - SCU_REG_SEC2_6FC"]
pub type Scuregsec26fcR = crate::BitReader;
#[doc = "Field `SCUREGSEC26FC` writer - SCU_REG_SEC2_6FC"]
pub type Scuregsec26fcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC2_680"]
    #[inline(always)]
    pub fn scuregsec2680(&self) -> Scuregsec2680R {
        Scuregsec2680R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC2_684"]
    #[inline(always)]
    pub fn scuregsec2684(&self) -> Scuregsec2684R {
        Scuregsec2684R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_SEC2_688"]
    #[inline(always)]
    pub fn scuregsec2688(&self) -> Scuregsec2688R {
        Scuregsec2688R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_SEC2_68C"]
    #[inline(always)]
    pub fn scuregsec268c(&self) -> Scuregsec268cR {
        Scuregsec268cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_REG_SEC2_6FC"]
    #[inline(always)]
    pub fn scuregsec26fc(&self) -> Scuregsec26fcR {
        Scuregsec26fcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC2_680"]
    #[inline(always)]
    pub fn scuregsec2680(&mut self) -> Scuregsec2680W<Scucb4Spec> {
        Scuregsec2680W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC2_684"]
    #[inline(always)]
    pub fn scuregsec2684(&mut self) -> Scuregsec2684W<Scucb4Spec> {
        Scuregsec2684W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_SEC2_688"]
    #[inline(always)]
    pub fn scuregsec2688(&mut self) -> Scuregsec2688W<Scucb4Spec> {
        Scuregsec2688W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_SEC2_68C"]
    #[inline(always)]
    pub fn scuregsec268c(&mut self) -> Scuregsec268cW<Scucb4Spec> {
        Scuregsec268cW::new(self, 3)
    }
    #[doc = "Bit 31 - SCU_REG_SEC2_6FC"]
    #[inline(always)]
    pub fn scuregsec26fc(&mut self) -> Scuregsec26fcW<Scucb4Spec> {
        Scuregsec26fcW::new(self, 31)
    }
}
#[doc = "Secure2 Control 14 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scucb4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scucb4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scucb4Spec;
impl crate::RegisterSpec for Scucb4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scucb4::R`](R) reader structure"]
impl crate::Readable for Scucb4Spec {}
#[doc = "`write(|w| ..)` method takes [`scucb4::W`](W) writer structure"]
impl crate::Writable for Scucb4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUCB4 to value 0"]
impl crate::Resettable for Scucb4Spec {}
