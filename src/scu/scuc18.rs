#[doc = "Register `SCUC18` reader"]
pub type R = crate::R<Scuc18Spec>;
#[doc = "Register `SCUC18` writer"]
pub type W = crate::W<Scuc18Spec>;
#[doc = "Field `SCUREGSEC1300` reader - SCU_REG_SEC1_300"]
pub type Scuregsec1300R = crate::BitReader;
#[doc = "Field `SCUREGSEC1300` writer - SCU_REG_SEC1_300"]
pub type Scuregsec1300W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1304` reader - SCU_REG_SEC1_304"]
pub type Scuregsec1304R = crate::BitReader;
#[doc = "Field `SCUREGSEC1304` writer - SCU_REG_SEC1_304"]
pub type Scuregsec1304W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `SCUREGSEC1310` reader - SCU_REG_SEC1_310"]
pub type Scuregsec1310R = crate::BitReader;
#[doc = "Field `SCUREGSEC1310` writer - SCU_REG_SEC1_310"]
pub type Scuregsec1310W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1314` reader - SCU_REG_SEC1_314"]
pub type Scuregsec1314R = crate::BitReader;
#[doc = "Field `SCUREGSEC1314` writer - SCU_REG_SEC1_314"]
pub type Scuregsec1314W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1318` reader - SCU_REG_SEC1_318"]
pub type Scuregsec1318R = crate::BitReader;
#[doc = "Field `SCUREGSEC1318` writer - SCU_REG_SEC1_318"]
pub type Scuregsec1318W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC131C` reader - SCU_REG_SEC1_31C"]
pub type Scuregsec131cR = crate::BitReader;
#[doc = "Field `SCUREGSEC131C` writer - SCU_REG_SEC1_31C"]
pub type Scuregsec131cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1320` reader - SCU_REG_SEC1_320"]
pub type Scuregsec1320R = crate::BitReader;
#[doc = "Field `SCUREGSEC1320` writer - SCU_REG_SEC1_320"]
pub type Scuregsec1320W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1324` reader - SCU_REG_SEC1_324"]
pub type Scuregsec1324R = crate::BitReader;
#[doc = "Field `SCUREGSEC1324` writer - SCU_REG_SEC1_324"]
pub type Scuregsec1324W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUREGSEC1330` reader - SCU_REG_SEC1_330"]
pub type Scuregsec1330R = crate::BitReader;
#[doc = "Field `SCUREGSEC1330` writer - SCU_REG_SEC1_330"]
pub type Scuregsec1330W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1334` reader - SCU_REG_SEC1_334"]
pub type Scuregsec1334R = crate::BitReader;
#[doc = "Field `SCUREGSEC1334` writer - SCU_REG_SEC1_334"]
pub type Scuregsec1334W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUREGSEC1340` reader - SCU_REG_SEC1_340"]
pub type Scuregsec1340R = crate::BitReader;
#[doc = "Field `SCUREGSEC1340` writer - SCU_REG_SEC1_340"]
pub type Scuregsec1340W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1344` reader - SCU_REG_SEC1_344"]
pub type Scuregsec1344R = crate::BitReader;
#[doc = "Field `SCUREGSEC1344` writer - SCU_REG_SEC1_344"]
pub type Scuregsec1344W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC1350` reader - SCU_REG_SEC1_350"]
pub type Scuregsec1350R = crate::BitReader;
#[doc = "Field `SCUREGSEC1350` writer - SCU_REG_SEC1_350"]
pub type Scuregsec1350W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1354` reader - SCU_REG_SEC1_354"]
pub type Scuregsec1354R = crate::BitReader;
#[doc = "Field `SCUREGSEC1354` writer - SCU_REG_SEC1_354"]
pub type Scuregsec1354W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC1_300"]
    #[inline(always)]
    pub fn scuregsec1300(&self) -> Scuregsec1300R {
        Scuregsec1300R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC1_304"]
    #[inline(always)]
    pub fn scuregsec1304(&self) -> Scuregsec1304R {
        Scuregsec1304R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SCU_REG_SEC1_310"]
    #[inline(always)]
    pub fn scuregsec1310(&self) -> Scuregsec1310R {
        Scuregsec1310R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_SEC1_314"]
    #[inline(always)]
    pub fn scuregsec1314(&self) -> Scuregsec1314R {
        Scuregsec1314R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_SEC1_318"]
    #[inline(always)]
    pub fn scuregsec1318(&self) -> Scuregsec1318R {
        Scuregsec1318R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_REG_SEC1_31C"]
    #[inline(always)]
    pub fn scuregsec131c(&self) -> Scuregsec131cR {
        Scuregsec131cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_SEC1_320"]
    #[inline(always)]
    pub fn scuregsec1320(&self) -> Scuregsec1320R {
        Scuregsec1320R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_SEC1_324"]
    #[inline(always)]
    pub fn scuregsec1324(&self) -> Scuregsec1324R {
        Scuregsec1324R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_SEC1_330"]
    #[inline(always)]
    pub fn scuregsec1330(&self) -> Scuregsec1330R {
        Scuregsec1330R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_SEC1_334"]
    #[inline(always)]
    pub fn scuregsec1334(&self) -> Scuregsec1334R {
        Scuregsec1334R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - SCU_REG_SEC1_340"]
    #[inline(always)]
    pub fn scuregsec1340(&self) -> Scuregsec1340R {
        Scuregsec1340R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_SEC1_344"]
    #[inline(always)]
    pub fn scuregsec1344(&self) -> Scuregsec1344R {
        Scuregsec1344R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - SCU_REG_SEC1_350"]
    #[inline(always)]
    pub fn scuregsec1350(&self) -> Scuregsec1350R {
        Scuregsec1350R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_SEC1_354"]
    #[inline(always)]
    pub fn scuregsec1354(&self) -> Scuregsec1354R {
        Scuregsec1354R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC1_300"]
    #[inline(always)]
    pub fn scuregsec1300(&mut self) -> Scuregsec1300W<Scuc18Spec> {
        Scuregsec1300W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC1_304"]
    #[inline(always)]
    pub fn scuregsec1304(&mut self) -> Scuregsec1304W<Scuc18Spec> {
        Scuregsec1304W::new(self, 1)
    }
    #[doc = "Bit 4 - SCU_REG_SEC1_310"]
    #[inline(always)]
    pub fn scuregsec1310(&mut self) -> Scuregsec1310W<Scuc18Spec> {
        Scuregsec1310W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_SEC1_314"]
    #[inline(always)]
    pub fn scuregsec1314(&mut self) -> Scuregsec1314W<Scuc18Spec> {
        Scuregsec1314W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_SEC1_318"]
    #[inline(always)]
    pub fn scuregsec1318(&mut self) -> Scuregsec1318W<Scuc18Spec> {
        Scuregsec1318W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_REG_SEC1_31C"]
    #[inline(always)]
    pub fn scuregsec131c(&mut self) -> Scuregsec131cW<Scuc18Spec> {
        Scuregsec131cW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_REG_SEC1_320"]
    #[inline(always)]
    pub fn scuregsec1320(&mut self) -> Scuregsec1320W<Scuc18Spec> {
        Scuregsec1320W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_SEC1_324"]
    #[inline(always)]
    pub fn scuregsec1324(&mut self) -> Scuregsec1324W<Scuc18Spec> {
        Scuregsec1324W::new(self, 9)
    }
    #[doc = "Bit 12 - SCU_REG_SEC1_330"]
    #[inline(always)]
    pub fn scuregsec1330(&mut self) -> Scuregsec1330W<Scuc18Spec> {
        Scuregsec1330W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_SEC1_334"]
    #[inline(always)]
    pub fn scuregsec1334(&mut self) -> Scuregsec1334W<Scuc18Spec> {
        Scuregsec1334W::new(self, 13)
    }
    #[doc = "Bit 16 - SCU_REG_SEC1_340"]
    #[inline(always)]
    pub fn scuregsec1340(&mut self) -> Scuregsec1340W<Scuc18Spec> {
        Scuregsec1340W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_SEC1_344"]
    #[inline(always)]
    pub fn scuregsec1344(&mut self) -> Scuregsec1344W<Scuc18Spec> {
        Scuregsec1344W::new(self, 17)
    }
    #[doc = "Bit 20 - SCU_REG_SEC1_350"]
    #[inline(always)]
    pub fn scuregsec1350(&mut self) -> Scuregsec1350W<Scuc18Spec> {
        Scuregsec1350W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_SEC1_354"]
    #[inline(always)]
    pub fn scuregsec1354(&mut self) -> Scuregsec1354W<Scuc18Spec> {
        Scuregsec1354W::new(self, 21)
    }
}
#[doc = "Secure1 Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc18Spec;
impl crate::RegisterSpec for Scuc18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc18::R`](R) reader structure"]
impl crate::Readable for Scuc18Spec {}
#[doc = "`write(|w| ..)` method takes [`scuc18::W`](W) writer structure"]
impl crate::Writable for Scuc18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC18 to value 0"]
impl crate::Resettable for Scuc18Spec {}
