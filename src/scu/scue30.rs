#[doc = "Register `SCUE30` reader"]
pub type R = crate::R<Scue30Spec>;
#[doc = "Register `SCUE30` writer"]
pub type W = crate::W<Scue30Spec>;
#[doc = "Field `SCUREGLOCK600` reader - SCU_REG_LOCK_600"]
pub type Scureglock600R = crate::BitReader;
#[doc = "Field `SCUREGLOCK600` writer - SCU_REG_LOCK_600"]
pub type Scureglock600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK604` reader - SCU_REG_LOCK_604"]
pub type Scureglock604R = crate::BitReader;
#[doc = "Field `SCUREGLOCK604` writer - SCU_REG_LOCK_604"]
pub type Scureglock604W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK608` reader - SCU_REG_LOCK_608"]
pub type Scureglock608R = crate::BitReader;
#[doc = "Field `SCUREGLOCK608` writer - SCU_REG_LOCK_608"]
pub type Scureglock608W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK60C` reader - SCU_REG_LOCK_60C"]
pub type Scureglock60cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK60C` writer - SCU_REG_LOCK_60C"]
pub type Scureglock60cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK610` reader - SCU_REG_LOCK_610"]
pub type Scureglock610R = crate::BitReader;
#[doc = "Field `SCUREGLOCK610` writer - SCU_REG_LOCK_610"]
pub type Scureglock610W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK614` reader - SCU_REG_LOCK_614"]
pub type Scureglock614R = crate::BitReader;
#[doc = "Field `SCUREGLOCK614` writer - SCU_REG_LOCK_614"]
pub type Scureglock614W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK618` reader - SCU_REG_LOCK_618"]
pub type Scureglock618R = crate::BitReader;
#[doc = "Field `SCUREGLOCK618` writer - SCU_REG_LOCK_618"]
pub type Scureglock618W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK61C` reader - SCU_REG_LOCK_61C"]
pub type Scureglock61cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK61C` writer - SCU_REG_LOCK_61C"]
pub type Scureglock61cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK620` reader - SCU_REG_LOCK_620"]
pub type Scureglock620R = crate::BitReader;
#[doc = "Field `SCUREGLOCK620` writer - SCU_REG_LOCK_620"]
pub type Scureglock620W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_600"]
    #[inline(always)]
    pub fn scureglock600(&self) -> Scureglock600R {
        Scureglock600R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_604"]
    #[inline(always)]
    pub fn scureglock604(&self) -> Scureglock604R {
        Scureglock604R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_608"]
    #[inline(always)]
    pub fn scureglock608(&self) -> Scureglock608R {
        Scureglock608R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_60C"]
    #[inline(always)]
    pub fn scureglock60c(&self) -> Scureglock60cR {
        Scureglock60cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_610"]
    #[inline(always)]
    pub fn scureglock610(&self) -> Scureglock610R {
        Scureglock610R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_614"]
    #[inline(always)]
    pub fn scureglock614(&self) -> Scureglock614R {
        Scureglock614R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_618"]
    #[inline(always)]
    pub fn scureglock618(&self) -> Scureglock618R {
        Scureglock618R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_61C"]
    #[inline(always)]
    pub fn scureglock61c(&self) -> Scureglock61cR {
        Scureglock61cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_620"]
    #[inline(always)]
    pub fn scureglock620(&self) -> Scureglock620R {
        Scureglock620R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_600"]
    #[inline(always)]
    pub fn scureglock600(&mut self) -> Scureglock600W<Scue30Spec> {
        Scureglock600W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_604"]
    #[inline(always)]
    pub fn scureglock604(&mut self) -> Scureglock604W<Scue30Spec> {
        Scureglock604W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_LOCK_608"]
    #[inline(always)]
    pub fn scureglock608(&mut self) -> Scureglock608W<Scue30Spec> {
        Scureglock608W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_LOCK_60C"]
    #[inline(always)]
    pub fn scureglock60c(&mut self) -> Scureglock60cW<Scue30Spec> {
        Scureglock60cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_610"]
    #[inline(always)]
    pub fn scureglock610(&mut self) -> Scureglock610W<Scue30Spec> {
        Scureglock610W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_614"]
    #[inline(always)]
    pub fn scureglock614(&mut self) -> Scureglock614W<Scue30Spec> {
        Scureglock614W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_618"]
    #[inline(always)]
    pub fn scureglock618(&mut self) -> Scureglock618W<Scue30Spec> {
        Scureglock618W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_61C"]
    #[inline(always)]
    pub fn scureglock61c(&mut self) -> Scureglock61cW<Scue30Spec> {
        Scureglock61cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_620"]
    #[inline(always)]
    pub fn scureglock620(&mut self) -> Scureglock620W<Scue30Spec> {
        Scureglock620W::new(self, 8)
    }
}
#[doc = "Write Protection 13 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue30Spec;
impl crate::RegisterSpec for Scue30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue30::R`](R) reader structure"]
impl crate::Readable for Scue30Spec {}
#[doc = "`write(|w| ..)` method takes [`scue30::W`](W) writer structure"]
impl crate::Writable for Scue30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE30 to value 0"]
impl crate::Resettable for Scue30Spec {}
