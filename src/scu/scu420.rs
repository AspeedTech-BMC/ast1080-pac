#[doc = "Register `SCU420` reader"]
pub type R = crate::R<Scu420Spec>;
#[doc = "Register `SCU420` writer"]
pub type W = crate::W<Scu420Spec>;
#[doc = "Field `SCUMUXIO064` reader - SCU_MUX_IO064"]
pub type Scumuxio064R = crate::FieldReader;
#[doc = "Field `SCUMUXIO064` writer - SCU_MUX_IO064"]
pub type Scumuxio064W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO065` reader - SCU_MUX_IO065"]
pub type Scumuxio065R = crate::FieldReader;
#[doc = "Field `SCUMUXIO065` writer - SCU_MUX_IO065"]
pub type Scumuxio065W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO066` reader - SCU_MUX_IO066"]
pub type Scumuxio066R = crate::FieldReader;
#[doc = "Field `SCUMUXIO066` writer - SCU_MUX_IO066"]
pub type Scumuxio066W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO067` reader - SCU_MUX_IO067"]
pub type Scumuxio067R = crate::FieldReader;
#[doc = "Field `SCUMUXIO067` writer - SCU_MUX_IO067"]
pub type Scumuxio067W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO068` reader - SCU_MUX_IO068"]
pub type Scumuxio068R = crate::FieldReader;
#[doc = "Field `SCUMUXIO068` writer - SCU_MUX_IO068"]
pub type Scumuxio068W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO069` reader - SCU_MUX_IO069"]
pub type Scumuxio069R = crate::FieldReader;
#[doc = "Field `SCUMUXIO069` writer - SCU_MUX_IO069"]
pub type Scumuxio069W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO070` reader - SCU_MUX_IO070"]
pub type Scumuxio070R = crate::FieldReader;
#[doc = "Field `SCUMUXIO070` writer - SCU_MUX_IO070"]
pub type Scumuxio070W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO071` reader - SCU_MUX_IO071"]
pub type Scumuxio071R = crate::FieldReader;
#[doc = "Field `SCUMUXIO071` writer - SCU_MUX_IO071"]
pub type Scumuxio071W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO064"]
    #[inline(always)]
    pub fn scumuxio064(&self) -> Scumuxio064R {
        Scumuxio064R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO065"]
    #[inline(always)]
    pub fn scumuxio065(&self) -> Scumuxio065R {
        Scumuxio065R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO066"]
    #[inline(always)]
    pub fn scumuxio066(&self) -> Scumuxio066R {
        Scumuxio066R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO067"]
    #[inline(always)]
    pub fn scumuxio067(&self) -> Scumuxio067R {
        Scumuxio067R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO068"]
    #[inline(always)]
    pub fn scumuxio068(&self) -> Scumuxio068R {
        Scumuxio068R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO069"]
    #[inline(always)]
    pub fn scumuxio069(&self) -> Scumuxio069R {
        Scumuxio069R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO070"]
    #[inline(always)]
    pub fn scumuxio070(&self) -> Scumuxio070R {
        Scumuxio070R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO071"]
    #[inline(always)]
    pub fn scumuxio071(&self) -> Scumuxio071R {
        Scumuxio071R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO064"]
    #[inline(always)]
    pub fn scumuxio064(&mut self) -> Scumuxio064W<Scu420Spec> {
        Scumuxio064W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO065"]
    #[inline(always)]
    pub fn scumuxio065(&mut self) -> Scumuxio065W<Scu420Spec> {
        Scumuxio065W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO066"]
    #[inline(always)]
    pub fn scumuxio066(&mut self) -> Scumuxio066W<Scu420Spec> {
        Scumuxio066W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO067"]
    #[inline(always)]
    pub fn scumuxio067(&mut self) -> Scumuxio067W<Scu420Spec> {
        Scumuxio067W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO068"]
    #[inline(always)]
    pub fn scumuxio068(&mut self) -> Scumuxio068W<Scu420Spec> {
        Scumuxio068W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO069"]
    #[inline(always)]
    pub fn scumuxio069(&mut self) -> Scumuxio069W<Scu420Spec> {
        Scumuxio069W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO070"]
    #[inline(always)]
    pub fn scumuxio070(&mut self) -> Scumuxio070W<Scu420Spec> {
        Scumuxio070W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO071"]
    #[inline(always)]
    pub fn scumuxio071(&mut self) -> Scumuxio071W<Scu420Spec> {
        Scumuxio071W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`scu420::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu420::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu420Spec;
impl crate::RegisterSpec for Scu420Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu420::R`](R) reader structure"]
impl crate::Readable for Scu420Spec {}
#[doc = "`write(|w| ..)` method takes [`scu420::W`](W) writer structure"]
impl crate::Writable for Scu420Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU420 to value 0"]
impl crate::Resettable for Scu420Spec {}
