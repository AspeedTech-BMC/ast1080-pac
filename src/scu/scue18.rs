#[doc = "Register `SCUE18` reader"]
pub type R = crate::R<Scue18Spec>;
#[doc = "Register `SCUE18` writer"]
pub type W = crate::W<Scue18Spec>;
#[doc = "Field `SCUREGLOCK300` reader - SCU_REG_LOCK_300"]
pub type Scureglock300R = crate::BitReader;
#[doc = "Field `SCUREGLOCK300` writer - SCU_REG_LOCK_300"]
pub type Scureglock300W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK304` reader - SCU_REG_LOCK_304"]
pub type Scureglock304R = crate::BitReader;
#[doc = "Field `SCUREGLOCK304` writer - SCU_REG_LOCK_304"]
pub type Scureglock304W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `SCUREGLOCK310` reader - SCU_REG_LOCK_310"]
pub type Scureglock310R = crate::BitReader;
#[doc = "Field `SCUREGLOCK310` writer - SCU_REG_LOCK_310"]
pub type Scureglock310W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK314` reader - SCU_REG_LOCK_314"]
pub type Scureglock314R = crate::BitReader;
#[doc = "Field `SCUREGLOCK314` writer - SCU_REG_LOCK_314"]
pub type Scureglock314W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK318` reader - SCU_REG_LOCK_318"]
pub type Scureglock318R = crate::BitReader;
#[doc = "Field `SCUREGLOCK318` writer - SCU_REG_LOCK_318"]
pub type Scureglock318W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK31C` reader - SCU_REG_LOCK_31C"]
pub type Scureglock31cR = crate::BitReader;
#[doc = "Field `SCUREGLOCK31C` writer - SCU_REG_LOCK_31C"]
pub type Scureglock31cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK320` reader - SCU_REG_LOCK_320"]
pub type Scureglock320R = crate::BitReader;
#[doc = "Field `SCUREGLOCK320` writer - SCU_REG_LOCK_320"]
pub type Scureglock320W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK324` reader - SCU_REG_LOCK_324"]
pub type Scureglock324R = crate::BitReader;
#[doc = "Field `SCUREGLOCK324` writer - SCU_REG_LOCK_324"]
pub type Scureglock324W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUREGLOCK330` reader - SCU_REG_LOCK_330"]
pub type Scureglock330R = crate::BitReader;
#[doc = "Field `SCUREGLOCK330` writer - SCU_REG_LOCK_330"]
pub type Scureglock330W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK334` reader - SCU_REG_LOCK_334"]
pub type Scureglock334R = crate::BitReader;
#[doc = "Field `SCUREGLOCK334` writer - SCU_REG_LOCK_334"]
pub type Scureglock334W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUREGLOCK340` reader - SCU_REG_LOCK_340"]
pub type Scureglock340R = crate::BitReader;
#[doc = "Field `SCUREGLOCK340` writer - SCU_REG_LOCK_340"]
pub type Scureglock340W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK344` reader - SCU_REG_LOCK_344"]
pub type Scureglock344R = crate::BitReader;
#[doc = "Field `SCUREGLOCK344` writer - SCU_REG_LOCK_344"]
pub type Scureglock344W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGLOCK350` reader - SCU_REG_LOCK_350"]
pub type Scureglock350R = crate::BitReader;
#[doc = "Field `SCUREGLOCK350` writer - SCU_REG_LOCK_350"]
pub type Scureglock350W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK354` reader - SCU_REG_LOCK_354"]
pub type Scureglock354R = crate::BitReader;
#[doc = "Field `SCUREGLOCK354` writer - SCU_REG_LOCK_354"]
pub type Scureglock354W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_300"]
    #[inline(always)]
    pub fn scureglock300(&self) -> Scureglock300R {
        Scureglock300R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_304"]
    #[inline(always)]
    pub fn scureglock304(&self) -> Scureglock304R {
        Scureglock304R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_310"]
    #[inline(always)]
    pub fn scureglock310(&self) -> Scureglock310R {
        Scureglock310R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_314"]
    #[inline(always)]
    pub fn scureglock314(&self) -> Scureglock314R {
        Scureglock314R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_318"]
    #[inline(always)]
    pub fn scureglock318(&self) -> Scureglock318R {
        Scureglock318R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_31C"]
    #[inline(always)]
    pub fn scureglock31c(&self) -> Scureglock31cR {
        Scureglock31cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_320"]
    #[inline(always)]
    pub fn scureglock320(&self) -> Scureglock320R {
        Scureglock320R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_LOCK_324"]
    #[inline(always)]
    pub fn scureglock324(&self) -> Scureglock324R {
        Scureglock324R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_LOCK_330"]
    #[inline(always)]
    pub fn scureglock330(&self) -> Scureglock330R {
        Scureglock330R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_334"]
    #[inline(always)]
    pub fn scureglock334(&self) -> Scureglock334R {
        Scureglock334R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - SCU_REG_LOCK_340"]
    #[inline(always)]
    pub fn scureglock340(&self) -> Scureglock340R {
        Scureglock340R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_LOCK_344"]
    #[inline(always)]
    pub fn scureglock344(&self) -> Scureglock344R {
        Scureglock344R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - SCU_REG_LOCK_350"]
    #[inline(always)]
    pub fn scureglock350(&self) -> Scureglock350R {
        Scureglock350R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_LOCK_354"]
    #[inline(always)]
    pub fn scureglock354(&self) -> Scureglock354R {
        Scureglock354R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_300"]
    #[inline(always)]
    pub fn scureglock300(&mut self) -> Scureglock300W<Scue18Spec> {
        Scureglock300W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_LOCK_304"]
    #[inline(always)]
    pub fn scureglock304(&mut self) -> Scureglock304W<Scue18Spec> {
        Scureglock304W::new(self, 1)
    }
    #[doc = "Bit 4 - SCU_REG_LOCK_310"]
    #[inline(always)]
    pub fn scureglock310(&mut self) -> Scureglock310W<Scue18Spec> {
        Scureglock310W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_LOCK_314"]
    #[inline(always)]
    pub fn scureglock314(&mut self) -> Scureglock314W<Scue18Spec> {
        Scureglock314W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_LOCK_318"]
    #[inline(always)]
    pub fn scureglock318(&mut self) -> Scureglock318W<Scue18Spec> {
        Scureglock318W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_LOCK_31C"]
    #[inline(always)]
    pub fn scureglock31c(&mut self) -> Scureglock31cW<Scue18Spec> {
        Scureglock31cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_LOCK_320"]
    #[inline(always)]
    pub fn scureglock320(&mut self) -> Scureglock320W<Scue18Spec> {
        Scureglock320W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_LOCK_324"]
    #[inline(always)]
    pub fn scureglock324(&mut self) -> Scureglock324W<Scue18Spec> {
        Scureglock324W::new(self, 9)
    }
    #[doc = "Bit 12 - SCU_REG_LOCK_330"]
    #[inline(always)]
    pub fn scureglock330(&mut self) -> Scureglock330W<Scue18Spec> {
        Scureglock330W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_334"]
    #[inline(always)]
    pub fn scureglock334(&mut self) -> Scureglock334W<Scue18Spec> {
        Scureglock334W::new(self, 13)
    }
    #[doc = "Bit 16 - SCU_REG_LOCK_340"]
    #[inline(always)]
    pub fn scureglock340(&mut self) -> Scureglock340W<Scue18Spec> {
        Scureglock340W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_LOCK_344"]
    #[inline(always)]
    pub fn scureglock344(&mut self) -> Scureglock344W<Scue18Spec> {
        Scureglock344W::new(self, 17)
    }
    #[doc = "Bit 20 - SCU_REG_LOCK_350"]
    #[inline(always)]
    pub fn scureglock350(&mut self) -> Scureglock350W<Scue18Spec> {
        Scureglock350W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_LOCK_354"]
    #[inline(always)]
    pub fn scureglock354(&mut self) -> Scureglock354W<Scue18Spec> {
        Scureglock354W::new(self, 21)
    }
}
#[doc = "Write Protection 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue18Spec;
impl crate::RegisterSpec for Scue18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue18::R`](R) reader structure"]
impl crate::Readable for Scue18Spec {}
#[doc = "`write(|w| ..)` method takes [`scue18::W`](W) writer structure"]
impl crate::Writable for Scue18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE18 to value 0"]
impl crate::Resettable for Scue18Spec {}
