#[doc = "Register `SCUCA8` reader"]
pub type R = crate::R<Scuca8Spec>;
#[doc = "Register `SCUCA8` writer"]
pub type W = crate::W<Scuca8Spec>;
#[doc = "Field `SCUREGSEC2540` reader - SCU_REG_SEC2_540"]
pub type Scuregsec2540R = crate::BitReader;
#[doc = "Field `SCUREGSEC2540` writer - SCU_REG_SEC2_540"]
pub type Scuregsec2540W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2544` reader - SCU_REG_SEC2_544"]
pub type Scuregsec2544R = crate::BitReader;
#[doc = "Field `SCUREGSEC2544` writer - SCU_REG_SEC2_544"]
pub type Scuregsec2544W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2548` reader - SCU_REG_SEC2_548"]
pub type Scuregsec2548R = crate::BitReader;
#[doc = "Field `SCUREGSEC2548` writer - SCU_REG_SEC2_548"]
pub type Scuregsec2548W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC254C` reader - SCU_REG_SEC2_54C"]
pub type Scuregsec254cR = crate::BitReader;
#[doc = "Field `SCUREGSEC254C` writer - SCU_REG_SEC2_54C"]
pub type Scuregsec254cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC257C` reader - SCU_REG_SEC2_57C"]
pub type Scuregsec257cR = crate::BitReader;
#[doc = "Field `SCUREGSEC257C` writer - SCU_REG_SEC2_57C"]
pub type Scuregsec257cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - SCU_REG_SEC2_540"]
    #[inline(always)]
    pub fn scuregsec2540(&self) -> Scuregsec2540R {
        Scuregsec2540R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_SEC2_544"]
    #[inline(always)]
    pub fn scuregsec2544(&self) -> Scuregsec2544R {
        Scuregsec2544R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_SEC2_548"]
    #[inline(always)]
    pub fn scuregsec2548(&self) -> Scuregsec2548R {
        Scuregsec2548R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_REG_SEC2_54C"]
    #[inline(always)]
    pub fn scuregsec254c(&self) -> Scuregsec254cR {
        Scuregsec254cR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_REG_SEC2_57C"]
    #[inline(always)]
    pub fn scuregsec257c(&self) -> Scuregsec257cR {
        Scuregsec257cR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - SCU_REG_SEC2_540"]
    #[inline(always)]
    pub fn scuregsec2540(&mut self) -> Scuregsec2540W<Scuca8Spec> {
        Scuregsec2540W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_SEC2_544"]
    #[inline(always)]
    pub fn scuregsec2544(&mut self) -> Scuregsec2544W<Scuca8Spec> {
        Scuregsec2544W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_SEC2_548"]
    #[inline(always)]
    pub fn scuregsec2548(&mut self) -> Scuregsec2548W<Scuca8Spec> {
        Scuregsec2548W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_REG_SEC2_54C"]
    #[inline(always)]
    pub fn scuregsec254c(&mut self) -> Scuregsec254cW<Scuca8Spec> {
        Scuregsec254cW::new(self, 19)
    }
    #[doc = "Bit 31 - SCU_REG_SEC2_57C"]
    #[inline(always)]
    pub fn scuregsec257c(&mut self) -> Scuregsec257cW<Scuca8Spec> {
        Scuregsec257cW::new(self, 31)
    }
}
#[doc = "Secure2 Control 11 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuca8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuca8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuca8Spec;
impl crate::RegisterSpec for Scuca8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuca8::R`](R) reader structure"]
impl crate::Readable for Scuca8Spec {}
#[doc = "`write(|w| ..)` method takes [`scuca8::W`](W) writer structure"]
impl crate::Writable for Scuca8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUCA8 to value 0"]
impl crate::Resettable for Scuca8Spec {}
