#[doc = "Register `SCUD18` reader"]
pub type R = crate::R<Scud18Spec>;
#[doc = "Register `SCUD18` writer"]
pub type W = crate::W<Scud18Spec>;
#[doc = "Field `SCUREGSEC3300` reader - SCU_REG_SEC3_300"]
pub type Scuregsec3300R = crate::BitReader;
#[doc = "Field `SCUREGSEC3300` writer - SCU_REG_SEC3_300"]
pub type Scuregsec3300W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3304` reader - SCU_REG_SEC3_304"]
pub type Scuregsec3304R = crate::BitReader;
#[doc = "Field `SCUREGSEC3304` writer - SCU_REG_SEC3_304"]
pub type Scuregsec3304W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `SCUREGSEC3310` reader - SCU_REG_SEC3_310"]
pub type Scuregsec3310R = crate::BitReader;
#[doc = "Field `SCUREGSEC3310` writer - SCU_REG_SEC3_310"]
pub type Scuregsec3310W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3314` reader - SCU_REG_SEC3_314"]
pub type Scuregsec3314R = crate::BitReader;
#[doc = "Field `SCUREGSEC3314` writer - SCU_REG_SEC3_314"]
pub type Scuregsec3314W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3318` reader - SCU_REG_SEC3_318"]
pub type Scuregsec3318R = crate::BitReader;
#[doc = "Field `SCUREGSEC3318` writer - SCU_REG_SEC3_318"]
pub type Scuregsec3318W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC331C` reader - SCU_REG_SEC3_31C"]
pub type Scuregsec331cR = crate::BitReader;
#[doc = "Field `SCUREGSEC331C` writer - SCU_REG_SEC3_31C"]
pub type Scuregsec331cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3320` reader - SCU_REG_SEC3_320"]
pub type Scuregsec3320R = crate::BitReader;
#[doc = "Field `SCUREGSEC3320` writer - SCU_REG_SEC3_320"]
pub type Scuregsec3320W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3324` reader - SCU_REG_SEC3_324"]
pub type Scuregsec3324R = crate::BitReader;
#[doc = "Field `SCUREGSEC3324` writer - SCU_REG_SEC3_324"]
pub type Scuregsec3324W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUREGSEC3330` reader - SCU_REG_SEC3_330"]
pub type Scuregsec3330R = crate::BitReader;
#[doc = "Field `SCUREGSEC3330` writer - SCU_REG_SEC3_330"]
pub type Scuregsec3330W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3334` reader - SCU_REG_SEC3_334"]
pub type Scuregsec3334R = crate::BitReader;
#[doc = "Field `SCUREGSEC3334` writer - SCU_REG_SEC3_334"]
pub type Scuregsec3334W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUREGSEC3340` reader - SCU_REG_SEC3_340"]
pub type Scuregsec3340R = crate::BitReader;
#[doc = "Field `SCUREGSEC3340` writer - SCU_REG_SEC3_340"]
pub type Scuregsec3340W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3344` reader - SCU_REG_SEC3_344"]
pub type Scuregsec3344R = crate::BitReader;
#[doc = "Field `SCUREGSEC3344` writer - SCU_REG_SEC3_344"]
pub type Scuregsec3344W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC3350` reader - SCU_REG_SEC3_350"]
pub type Scuregsec3350R = crate::BitReader;
#[doc = "Field `SCUREGSEC3350` writer - SCU_REG_SEC3_350"]
pub type Scuregsec3350W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3354` reader - SCU_REG_SEC3_354"]
pub type Scuregsec3354R = crate::BitReader;
#[doc = "Field `SCUREGSEC3354` writer - SCU_REG_SEC3_354"]
pub type Scuregsec3354W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC3_300"]
    #[inline(always)]
    pub fn scuregsec3300(&self) -> Scuregsec3300R {
        Scuregsec3300R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC3_304"]
    #[inline(always)]
    pub fn scuregsec3304(&self) -> Scuregsec3304R {
        Scuregsec3304R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SCU_REG_SEC3_310"]
    #[inline(always)]
    pub fn scuregsec3310(&self) -> Scuregsec3310R {
        Scuregsec3310R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_SEC3_314"]
    #[inline(always)]
    pub fn scuregsec3314(&self) -> Scuregsec3314R {
        Scuregsec3314R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_SEC3_318"]
    #[inline(always)]
    pub fn scuregsec3318(&self) -> Scuregsec3318R {
        Scuregsec3318R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_SEC3_31C"]
    #[inline(always)]
    pub fn scuregsec331c(&self) -> Scuregsec331cR {
        Scuregsec331cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_SEC3_320"]
    #[inline(always)]
    pub fn scuregsec3320(&self) -> Scuregsec3320R {
        Scuregsec3320R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_SEC3_324"]
    #[inline(always)]
    pub fn scuregsec3324(&self) -> Scuregsec3324R {
        Scuregsec3324R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_SEC3_330"]
    #[inline(always)]
    pub fn scuregsec3330(&self) -> Scuregsec3330R {
        Scuregsec3330R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_SEC3_334"]
    #[inline(always)]
    pub fn scuregsec3334(&self) -> Scuregsec3334R {
        Scuregsec3334R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - SCU_REG_SEC3_340"]
    #[inline(always)]
    pub fn scuregsec3340(&self) -> Scuregsec3340R {
        Scuregsec3340R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_SEC3_344"]
    #[inline(always)]
    pub fn scuregsec3344(&self) -> Scuregsec3344R {
        Scuregsec3344R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - SCU_REG_SEC3_350"]
    #[inline(always)]
    pub fn scuregsec3350(&self) -> Scuregsec3350R {
        Scuregsec3350R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_SEC3_354"]
    #[inline(always)]
    pub fn scuregsec3354(&self) -> Scuregsec3354R {
        Scuregsec3354R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC3_300"]
    #[inline(always)]
    pub fn scuregsec3300(&mut self) -> Scuregsec3300W<Scud18Spec> {
        Scuregsec3300W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC3_304"]
    #[inline(always)]
    pub fn scuregsec3304(&mut self) -> Scuregsec3304W<Scud18Spec> {
        Scuregsec3304W::new(self, 1)
    }
    #[doc = "Bit 4 - SCU_REG_SEC3_310"]
    #[inline(always)]
    pub fn scuregsec3310(&mut self) -> Scuregsec3310W<Scud18Spec> {
        Scuregsec3310W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_SEC3_314"]
    #[inline(always)]
    pub fn scuregsec3314(&mut self) -> Scuregsec3314W<Scud18Spec> {
        Scuregsec3314W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_SEC3_318"]
    #[inline(always)]
    pub fn scuregsec3318(&mut self) -> Scuregsec3318W<Scud18Spec> {
        Scuregsec3318W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_SEC3_31C"]
    #[inline(always)]
    pub fn scuregsec331c(&mut self) -> Scuregsec331cW<Scud18Spec> {
        Scuregsec331cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_SEC3_320"]
    #[inline(always)]
    pub fn scuregsec3320(&mut self) -> Scuregsec3320W<Scud18Spec> {
        Scuregsec3320W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_SEC3_324"]
    #[inline(always)]
    pub fn scuregsec3324(&mut self) -> Scuregsec3324W<Scud18Spec> {
        Scuregsec3324W::new(self, 9)
    }
    #[doc = "Bit 12 - SCU_REG_SEC3_330"]
    #[inline(always)]
    pub fn scuregsec3330(&mut self) -> Scuregsec3330W<Scud18Spec> {
        Scuregsec3330W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_SEC3_334"]
    #[inline(always)]
    pub fn scuregsec3334(&mut self) -> Scuregsec3334W<Scud18Spec> {
        Scuregsec3334W::new(self, 13)
    }
    #[doc = "Bit 16 - SCU_REG_SEC3_340"]
    #[inline(always)]
    pub fn scuregsec3340(&mut self) -> Scuregsec3340W<Scud18Spec> {
        Scuregsec3340W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_SEC3_344"]
    #[inline(always)]
    pub fn scuregsec3344(&mut self) -> Scuregsec3344W<Scud18Spec> {
        Scuregsec3344W::new(self, 17)
    }
    #[doc = "Bit 20 - SCU_REG_SEC3_350"]
    #[inline(always)]
    pub fn scuregsec3350(&mut self) -> Scuregsec3350W<Scud18Spec> {
        Scuregsec3350W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_SEC3_354"]
    #[inline(always)]
    pub fn scuregsec3354(&mut self) -> Scuregsec3354W<Scud18Spec> {
        Scuregsec3354W::new(self, 21)
    }
}
#[doc = "Secure3 Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scud18Spec;
impl crate::RegisterSpec for Scud18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scud18::R`](R) reader structure"]
impl crate::Readable for Scud18Spec {}
#[doc = "`write(|w| ..)` method takes [`scud18::W`](W) writer structure"]
impl crate::Writable for Scud18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUD18 to value 0"]
impl crate::Resettable for Scud18Spec {}
