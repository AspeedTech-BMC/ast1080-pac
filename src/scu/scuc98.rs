#[doc = "Register `SCUC98` reader"]
pub type R = crate::R<Scuc98Spec>;
#[doc = "Register `SCUC98` writer"]
pub type W = crate::W<Scuc98Spec>;
#[doc = "Field `SCUREGSEC2300` reader - SCU_REG_SEC2_300"]
pub type Scuregsec2300R = crate::BitReader;
#[doc = "Field `SCUREGSEC2300` writer - SCU_REG_SEC2_300"]
pub type Scuregsec2300W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2304` reader - SCU_REG_SEC2_304"]
pub type Scuregsec2304R = crate::BitReader;
#[doc = "Field `SCUREGSEC2304` writer - SCU_REG_SEC2_304"]
pub type Scuregsec2304W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `SCUREGSEC2310` reader - SCU_REG_SEC2_310"]
pub type Scuregsec2310R = crate::BitReader;
#[doc = "Field `SCUREGSEC2310` writer - SCU_REG_SEC2_310"]
pub type Scuregsec2310W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2314` reader - SCU_REG_SEC2_314"]
pub type Scuregsec2314R = crate::BitReader;
#[doc = "Field `SCUREGSEC2314` writer - SCU_REG_SEC2_314"]
pub type Scuregsec2314W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2318` reader - SCU_REG_SEC2_318"]
pub type Scuregsec2318R = crate::BitReader;
#[doc = "Field `SCUREGSEC2318` writer - SCU_REG_SEC2_318"]
pub type Scuregsec2318W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC231C` reader - SCU_REG_SEC2_31C"]
pub type Scuregsec231cR = crate::BitReader;
#[doc = "Field `SCUREGSEC231C` writer - SCU_REG_SEC2_31C"]
pub type Scuregsec231cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2320` reader - SCU_REG_SEC2_320"]
pub type Scuregsec2320R = crate::BitReader;
#[doc = "Field `SCUREGSEC2320` writer - SCU_REG_SEC2_320"]
pub type Scuregsec2320W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2324` reader - SCU_REG_SEC2_324"]
pub type Scuregsec2324R = crate::BitReader;
#[doc = "Field `SCUREGSEC2324` writer - SCU_REG_SEC2_324"]
pub type Scuregsec2324W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUREGSEC2330` reader - SCU_REG_SEC2_330"]
pub type Scuregsec2330R = crate::BitReader;
#[doc = "Field `SCUREGSEC2330` writer - SCU_REG_SEC2_330"]
pub type Scuregsec2330W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2334` reader - SCU_REG_SEC2_334"]
pub type Scuregsec2334R = crate::BitReader;
#[doc = "Field `SCUREGSEC2334` writer - SCU_REG_SEC2_334"]
pub type Scuregsec2334W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUREGSEC2340` reader - SCU_REG_SEC2_340"]
pub type Scuregsec2340R = crate::BitReader;
#[doc = "Field `SCUREGSEC2340` writer - SCU_REG_SEC2_340"]
pub type Scuregsec2340W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2344` reader - SCU_REG_SEC2_344"]
pub type Scuregsec2344R = crate::BitReader;
#[doc = "Field `SCUREGSEC2344` writer - SCU_REG_SEC2_344"]
pub type Scuregsec2344W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC2350` reader - SCU_REG_SEC2_350"]
pub type Scuregsec2350R = crate::BitReader;
#[doc = "Field `SCUREGSEC2350` writer - SCU_REG_SEC2_350"]
pub type Scuregsec2350W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2354` reader - SCU_REG_SEC2_354"]
pub type Scuregsec2354R = crate::BitReader;
#[doc = "Field `SCUREGSEC2354` writer - SCU_REG_SEC2_354"]
pub type Scuregsec2354W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC2_300"]
    #[inline(always)]
    pub fn scuregsec2300(&self) -> Scuregsec2300R {
        Scuregsec2300R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC2_304"]
    #[inline(always)]
    pub fn scuregsec2304(&self) -> Scuregsec2304R {
        Scuregsec2304R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SCU_REG_SEC2_310"]
    #[inline(always)]
    pub fn scuregsec2310(&self) -> Scuregsec2310R {
        Scuregsec2310R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_SEC2_314"]
    #[inline(always)]
    pub fn scuregsec2314(&self) -> Scuregsec2314R {
        Scuregsec2314R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_SEC2_318"]
    #[inline(always)]
    pub fn scuregsec2318(&self) -> Scuregsec2318R {
        Scuregsec2318R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_SEC2_31C"]
    #[inline(always)]
    pub fn scuregsec231c(&self) -> Scuregsec231cR {
        Scuregsec231cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_SEC2_320"]
    #[inline(always)]
    pub fn scuregsec2320(&self) -> Scuregsec2320R {
        Scuregsec2320R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_SEC2_324"]
    #[inline(always)]
    pub fn scuregsec2324(&self) -> Scuregsec2324R {
        Scuregsec2324R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_SEC2_330"]
    #[inline(always)]
    pub fn scuregsec2330(&self) -> Scuregsec2330R {
        Scuregsec2330R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_SEC2_334"]
    #[inline(always)]
    pub fn scuregsec2334(&self) -> Scuregsec2334R {
        Scuregsec2334R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - SCU_REG_SEC2_340"]
    #[inline(always)]
    pub fn scuregsec2340(&self) -> Scuregsec2340R {
        Scuregsec2340R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_SEC2_344"]
    #[inline(always)]
    pub fn scuregsec2344(&self) -> Scuregsec2344R {
        Scuregsec2344R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - SCU_REG_SEC2_350"]
    #[inline(always)]
    pub fn scuregsec2350(&self) -> Scuregsec2350R {
        Scuregsec2350R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_SEC2_354"]
    #[inline(always)]
    pub fn scuregsec2354(&self) -> Scuregsec2354R {
        Scuregsec2354R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC2_300"]
    #[inline(always)]
    pub fn scuregsec2300(&mut self) -> Scuregsec2300W<Scuc98Spec> {
        Scuregsec2300W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC2_304"]
    #[inline(always)]
    pub fn scuregsec2304(&mut self) -> Scuregsec2304W<Scuc98Spec> {
        Scuregsec2304W::new(self, 1)
    }
    #[doc = "Bit 4 - SCU_REG_SEC2_310"]
    #[inline(always)]
    pub fn scuregsec2310(&mut self) -> Scuregsec2310W<Scuc98Spec> {
        Scuregsec2310W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_SEC2_314"]
    #[inline(always)]
    pub fn scuregsec2314(&mut self) -> Scuregsec2314W<Scuc98Spec> {
        Scuregsec2314W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_SEC2_318"]
    #[inline(always)]
    pub fn scuregsec2318(&mut self) -> Scuregsec2318W<Scuc98Spec> {
        Scuregsec2318W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_SEC2_31C"]
    #[inline(always)]
    pub fn scuregsec231c(&mut self) -> Scuregsec231cW<Scuc98Spec> {
        Scuregsec231cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_SEC2_320"]
    #[inline(always)]
    pub fn scuregsec2320(&mut self) -> Scuregsec2320W<Scuc98Spec> {
        Scuregsec2320W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_SEC2_324"]
    #[inline(always)]
    pub fn scuregsec2324(&mut self) -> Scuregsec2324W<Scuc98Spec> {
        Scuregsec2324W::new(self, 9)
    }
    #[doc = "Bit 12 - SCU_REG_SEC2_330"]
    #[inline(always)]
    pub fn scuregsec2330(&mut self) -> Scuregsec2330W<Scuc98Spec> {
        Scuregsec2330W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_SEC2_334"]
    #[inline(always)]
    pub fn scuregsec2334(&mut self) -> Scuregsec2334W<Scuc98Spec> {
        Scuregsec2334W::new(self, 13)
    }
    #[doc = "Bit 16 - SCU_REG_SEC2_340"]
    #[inline(always)]
    pub fn scuregsec2340(&mut self) -> Scuregsec2340W<Scuc98Spec> {
        Scuregsec2340W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_SEC2_344"]
    #[inline(always)]
    pub fn scuregsec2344(&mut self) -> Scuregsec2344W<Scuc98Spec> {
        Scuregsec2344W::new(self, 17)
    }
    #[doc = "Bit 20 - SCU_REG_SEC2_350"]
    #[inline(always)]
    pub fn scuregsec2350(&mut self) -> Scuregsec2350W<Scuc98Spec> {
        Scuregsec2350W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_SEC2_354"]
    #[inline(always)]
    pub fn scuregsec2354(&mut self) -> Scuregsec2354W<Scuc98Spec> {
        Scuregsec2354W::new(self, 21)
    }
}
#[doc = "Secure2 Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc98::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc98::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc98Spec;
impl crate::RegisterSpec for Scuc98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc98::R`](R) reader structure"]
impl crate::Readable for Scuc98Spec {}
#[doc = "`write(|w| ..)` method takes [`scuc98::W`](W) writer structure"]
impl crate::Writable for Scuc98Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC98 to value 0"]
impl crate::Resettable for Scuc98Spec {}
