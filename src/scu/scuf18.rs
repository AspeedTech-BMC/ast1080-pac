#[doc = "Register `SCUF18` reader"]
pub type R = crate::R<Scuf18Spec>;
#[doc = "Register `SCUF18` writer"]
pub type W = crate::W<Scuf18Spec>;
#[doc = "Field `SCUREGRST300` reader - SCU_REG_RST_300"]
pub type Scuregrst300R = crate::BitReader;
#[doc = "Field `SCUREGRST300` writer - SCU_REG_RST_300"]
pub type Scuregrst300W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST304` reader - SCU_REG_RST_304"]
pub type Scuregrst304R = crate::BitReader;
#[doc = "Field `SCUREGRST304` writer - SCU_REG_RST_304"]
pub type Scuregrst304W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUREGRST310` reader - SCU_REG_RST_310"]
pub type Scuregrst310R = crate::BitReader;
#[doc = "Field `SCUREGRST310` writer - SCU_REG_RST_310"]
pub type Scuregrst310W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST314` reader - SCU_REG_RST_314"]
pub type Scuregrst314R = crate::BitReader;
#[doc = "Field `SCUREGRST314` writer - SCU_REG_RST_314"]
pub type Scuregrst314W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST318` reader - SCU_REG_RST_318"]
pub type Scuregrst318R = crate::BitReader;
#[doc = "Field `SCUREGRST318` writer - SCU_REG_RST_318"]
pub type Scuregrst318W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST31C` reader - SCU_REG_RST_31C"]
pub type Scuregrst31cR = crate::BitReader;
#[doc = "Field `SCUREGRST31C` writer - SCU_REG_RST_31C"]
pub type Scuregrst31cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST320` reader - SCU_REG_RST_320"]
pub type Scuregrst320R = crate::BitReader;
#[doc = "Field `SCUREGRST320` writer - SCU_REG_RST_320"]
pub type Scuregrst320W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST324` reader - SCU_REG_RST_324"]
pub type Scuregrst324R = crate::BitReader;
#[doc = "Field `SCUREGRST324` writer - SCU_REG_RST_324"]
pub type Scuregrst324W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUREGRST330` reader - SCU_REG_RST_330"]
pub type Scuregrst330R = crate::BitReader;
#[doc = "Field `SCUREGRST330` writer - SCU_REG_RST_330"]
pub type Scuregrst330W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST334` reader - SCU_REG_RST_334"]
pub type Scuregrst334R = crate::BitReader;
#[doc = "Field `SCUREGRST334` writer - SCU_REG_RST_334"]
pub type Scuregrst334W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGRST340` reader - SCU_REG_RST_340"]
pub type Scuregrst340R = crate::BitReader;
#[doc = "Field `SCUREGRST340` writer - SCU_REG_RST_340"]
pub type Scuregrst340W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST344` reader - SCU_REG_RST_344"]
pub type Scuregrst344R = crate::BitReader;
#[doc = "Field `SCUREGRST344` writer - SCU_REG_RST_344"]
pub type Scuregrst344W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_RST_300"]
    #[inline(always)]
    pub fn scuregrst300(&self) -> Scuregrst300R {
        Scuregrst300R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_304"]
    #[inline(always)]
    pub fn scuregrst304(&self) -> Scuregrst304R {
        Scuregrst304R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SCU_REG_RST_310"]
    #[inline(always)]
    pub fn scuregrst310(&self) -> Scuregrst310R {
        Scuregrst310R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_RST_314"]
    #[inline(always)]
    pub fn scuregrst314(&self) -> Scuregrst314R {
        Scuregrst314R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_RST_318"]
    #[inline(always)]
    pub fn scuregrst318(&self) -> Scuregrst318R {
        Scuregrst318R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_RST_31C"]
    #[inline(always)]
    pub fn scuregrst31c(&self) -> Scuregrst31cR {
        Scuregrst31cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_RST_320"]
    #[inline(always)]
    pub fn scuregrst320(&self) -> Scuregrst320R {
        Scuregrst320R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_RST_324"]
    #[inline(always)]
    pub fn scuregrst324(&self) -> Scuregrst324R {
        Scuregrst324R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_RST_330"]
    #[inline(always)]
    pub fn scuregrst330(&self) -> Scuregrst330R {
        Scuregrst330R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_RST_334"]
    #[inline(always)]
    pub fn scuregrst334(&self) -> Scuregrst334R {
        Scuregrst334R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - SCU_REG_RST_340"]
    #[inline(always)]
    pub fn scuregrst340(&self) -> Scuregrst340R {
        Scuregrst340R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_RST_344"]
    #[inline(always)]
    pub fn scuregrst344(&self) -> Scuregrst344R {
        Scuregrst344R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_RST_300"]
    #[inline(always)]
    pub fn scuregrst300(&mut self) -> Scuregrst300W<Scuf18Spec> {
        Scuregrst300W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_304"]
    #[inline(always)]
    pub fn scuregrst304(&mut self) -> Scuregrst304W<Scuf18Spec> {
        Scuregrst304W::new(self, 1)
    }
    #[doc = "Bit 4 - SCU_REG_RST_310"]
    #[inline(always)]
    pub fn scuregrst310(&mut self) -> Scuregrst310W<Scuf18Spec> {
        Scuregrst310W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_RST_314"]
    #[inline(always)]
    pub fn scuregrst314(&mut self) -> Scuregrst314W<Scuf18Spec> {
        Scuregrst314W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_RST_318"]
    #[inline(always)]
    pub fn scuregrst318(&mut self) -> Scuregrst318W<Scuf18Spec> {
        Scuregrst318W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_RST_31C"]
    #[inline(always)]
    pub fn scuregrst31c(&mut self) -> Scuregrst31cW<Scuf18Spec> {
        Scuregrst31cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_RST_320"]
    #[inline(always)]
    pub fn scuregrst320(&mut self) -> Scuregrst320W<Scuf18Spec> {
        Scuregrst320W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_RST_324"]
    #[inline(always)]
    pub fn scuregrst324(&mut self) -> Scuregrst324W<Scuf18Spec> {
        Scuregrst324W::new(self, 9)
    }
    #[doc = "Bit 12 - SCU_REG_RST_330"]
    #[inline(always)]
    pub fn scuregrst330(&mut self) -> Scuregrst330W<Scuf18Spec> {
        Scuregrst330W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_RST_334"]
    #[inline(always)]
    pub fn scuregrst334(&mut self) -> Scuregrst334W<Scuf18Spec> {
        Scuregrst334W::new(self, 13)
    }
    #[doc = "Bit 16 - SCU_REG_RST_340"]
    #[inline(always)]
    pub fn scuregrst340(&mut self) -> Scuregrst340W<Scuf18Spec> {
        Scuregrst340W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_RST_344"]
    #[inline(always)]
    pub fn scuregrst344(&mut self) -> Scuregrst344W<Scuf18Spec> {
        Scuregrst344W::new(self, 17)
    }
}
#[doc = "Reset Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf18Spec;
impl crate::RegisterSpec for Scuf18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf18::R`](R) reader structure"]
impl crate::Readable for Scuf18Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf18::W`](W) writer structure"]
impl crate::Writable for Scuf18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF18 to value 0"]
impl crate::Resettable for Scuf18Spec {}
