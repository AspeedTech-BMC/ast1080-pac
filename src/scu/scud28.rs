#[doc = "Register `SCUD28` reader"]
pub type R = crate::R<Scud28Spec>;
#[doc = "Register `SCUD28` writer"]
pub type W = crate::W<Scud28Spec>;
#[doc = "Field `SCUREGSEC3540` reader - SCU_REG_SEC3_540"]
pub type Scuregsec3540R = crate::BitReader;
#[doc = "Field `SCUREGSEC3540` writer - SCU_REG_SEC3_540"]
pub type Scuregsec3540W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3544` reader - SCU_REG_SEC3_544"]
pub type Scuregsec3544R = crate::BitReader;
#[doc = "Field `SCUREGSEC3544` writer - SCU_REG_SEC3_544"]
pub type Scuregsec3544W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3548` reader - SCU_REG_SEC3_548"]
pub type Scuregsec3548R = crate::BitReader;
#[doc = "Field `SCUREGSEC3548` writer - SCU_REG_SEC3_548"]
pub type Scuregsec3548W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC354C` reader - SCU_REG_SEC3_54C"]
pub type Scuregsec354cR = crate::BitReader;
#[doc = "Field `SCUREGSEC354C` writer - SCU_REG_SEC3_54C"]
pub type Scuregsec354cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC357C` reader - SCU_REG_SEC3_57C"]
pub type Scuregsec357cR = crate::BitReader;
#[doc = "Field `SCUREGSEC357C` writer - SCU_REG_SEC3_57C"]
pub type Scuregsec357cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - SCU_REG_SEC3_540"]
    #[inline(always)]
    pub fn scuregsec3540(&self) -> Scuregsec3540R {
        Scuregsec3540R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_SEC3_544"]
    #[inline(always)]
    pub fn scuregsec3544(&self) -> Scuregsec3544R {
        Scuregsec3544R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_SEC3_548"]
    #[inline(always)]
    pub fn scuregsec3548(&self) -> Scuregsec3548R {
        Scuregsec3548R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_REG_SEC3_54C"]
    #[inline(always)]
    pub fn scuregsec354c(&self) -> Scuregsec354cR {
        Scuregsec354cR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_REG_SEC3_57C"]
    #[inline(always)]
    pub fn scuregsec357c(&self) -> Scuregsec357cR {
        Scuregsec357cR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - SCU_REG_SEC3_540"]
    #[inline(always)]
    pub fn scuregsec3540(&mut self) -> Scuregsec3540W<Scud28Spec> {
        Scuregsec3540W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_SEC3_544"]
    #[inline(always)]
    pub fn scuregsec3544(&mut self) -> Scuregsec3544W<Scud28Spec> {
        Scuregsec3544W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_SEC3_548"]
    #[inline(always)]
    pub fn scuregsec3548(&mut self) -> Scuregsec3548W<Scud28Spec> {
        Scuregsec3548W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_REG_SEC3_54C"]
    #[inline(always)]
    pub fn scuregsec354c(&mut self) -> Scuregsec354cW<Scud28Spec> {
        Scuregsec354cW::new(self, 19)
    }
    #[doc = "Bit 31 - SCU_REG_SEC3_57C"]
    #[inline(always)]
    pub fn scuregsec357c(&mut self) -> Scuregsec357cW<Scud28Spec> {
        Scuregsec357cW::new(self, 31)
    }
}
#[doc = "Secure3 Control 11 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scud28Spec;
impl crate::RegisterSpec for Scud28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scud28::R`](R) reader structure"]
impl crate::Readable for Scud28Spec {}
#[doc = "`write(|w| ..)` method takes [`scud28::W`](W) writer structure"]
impl crate::Writable for Scud28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUD28 to value 0"]
impl crate::Resettable for Scud28Spec {}
