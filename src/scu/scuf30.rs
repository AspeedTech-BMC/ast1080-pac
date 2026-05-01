#[doc = "Register `SCUF30` reader"]
pub type R = crate::R<Scuf30Spec>;
#[doc = "Register `SCUF30` writer"]
pub type W = crate::W<Scuf30Spec>;
#[doc = "Field `SCUREGRST600` reader - SCU_REG_RST_600"]
pub type Scuregrst600R = crate::BitReader;
#[doc = "Field `SCUREGRST600` writer - SCU_REG_RST_600"]
pub type Scuregrst600W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST604` reader - SCU_REG_RST_604"]
pub type Scuregrst604R = crate::BitReader;
#[doc = "Field `SCUREGRST604` writer - SCU_REG_RST_604"]
pub type Scuregrst604W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST608` reader - SCU_REG_RST_608"]
pub type Scuregrst608R = crate::BitReader;
#[doc = "Field `SCUREGRST608` writer - SCU_REG_RST_608"]
pub type Scuregrst608W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST60C` reader - SCU_REG_RST_60C"]
pub type Scuregrst60cR = crate::BitReader;
#[doc = "Field `SCUREGRST60C` writer - SCU_REG_RST_60C"]
pub type Scuregrst60cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST610` reader - SCU_REG_RST_610"]
pub type Scuregrst610R = crate::BitReader;
#[doc = "Field `SCUREGRST610` writer - SCU_REG_RST_610"]
pub type Scuregrst610W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST614` reader - SCU_REG_RST_614"]
pub type Scuregrst614R = crate::BitReader;
#[doc = "Field `SCUREGRST614` writer - SCU_REG_RST_614"]
pub type Scuregrst614W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST618` reader - SCU_REG_RST_618"]
pub type Scuregrst618R = crate::BitReader;
#[doc = "Field `SCUREGRST618` writer - SCU_REG_RST_618"]
pub type Scuregrst618W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST61C` reader - SCU_REG_RST_61C"]
pub type Scuregrst61cR = crate::BitReader;
#[doc = "Field `SCUREGRST61C` writer - SCU_REG_RST_61C"]
pub type Scuregrst61cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST620` reader - SCU_REG_RST_620"]
pub type Scuregrst620R = crate::BitReader;
#[doc = "Field `SCUREGRST620` writer - SCU_REG_RST_620"]
pub type Scuregrst620W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_RST_600"]
    #[inline(always)]
    pub fn scuregrst600(&self) -> Scuregrst600R {
        Scuregrst600R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_604"]
    #[inline(always)]
    pub fn scuregrst604(&self) -> Scuregrst604R {
        Scuregrst604R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_RST_608"]
    #[inline(always)]
    pub fn scuregrst608(&self) -> Scuregrst608R {
        Scuregrst608R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_RST_60C"]
    #[inline(always)]
    pub fn scuregrst60c(&self) -> Scuregrst60cR {
        Scuregrst60cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_RST_610"]
    #[inline(always)]
    pub fn scuregrst610(&self) -> Scuregrst610R {
        Scuregrst610R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_RST_614"]
    #[inline(always)]
    pub fn scuregrst614(&self) -> Scuregrst614R {
        Scuregrst614R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_RST_618"]
    #[inline(always)]
    pub fn scuregrst618(&self) -> Scuregrst618R {
        Scuregrst618R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_RST_61C"]
    #[inline(always)]
    pub fn scuregrst61c(&self) -> Scuregrst61cR {
        Scuregrst61cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_RST_620"]
    #[inline(always)]
    pub fn scuregrst620(&self) -> Scuregrst620R {
        Scuregrst620R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_RST_600"]
    #[inline(always)]
    pub fn scuregrst600(&mut self) -> Scuregrst600W<Scuf30Spec> {
        Scuregrst600W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_604"]
    #[inline(always)]
    pub fn scuregrst604(&mut self) -> Scuregrst604W<Scuf30Spec> {
        Scuregrst604W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_RST_608"]
    #[inline(always)]
    pub fn scuregrst608(&mut self) -> Scuregrst608W<Scuf30Spec> {
        Scuregrst608W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_RST_60C"]
    #[inline(always)]
    pub fn scuregrst60c(&mut self) -> Scuregrst60cW<Scuf30Spec> {
        Scuregrst60cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_RST_610"]
    #[inline(always)]
    pub fn scuregrst610(&mut self) -> Scuregrst610W<Scuf30Spec> {
        Scuregrst610W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_RST_614"]
    #[inline(always)]
    pub fn scuregrst614(&mut self) -> Scuregrst614W<Scuf30Spec> {
        Scuregrst614W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_RST_618"]
    #[inline(always)]
    pub fn scuregrst618(&mut self) -> Scuregrst618W<Scuf30Spec> {
        Scuregrst618W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_RST_61C"]
    #[inline(always)]
    pub fn scuregrst61c(&mut self) -> Scuregrst61cW<Scuf30Spec> {
        Scuregrst61cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_RST_620"]
    #[inline(always)]
    pub fn scuregrst620(&mut self) -> Scuregrst620W<Scuf30Spec> {
        Scuregrst620W::new(self, 8)
    }
}
#[doc = "Reset Control 13 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf30Spec;
impl crate::RegisterSpec for Scuf30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf30::R`](R) reader structure"]
impl crate::Readable for Scuf30Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf30::W`](W) writer structure"]
impl crate::Writable for Scuf30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF30 to value 0"]
impl crate::Resettable for Scuf30Spec {}
