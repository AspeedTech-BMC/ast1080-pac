#[doc = "Register `SCU444` reader"]
pub type R = crate::R<Scu444Spec>;
#[doc = "Register `SCU444` writer"]
pub type W = crate::W<Scu444Spec>;
#[doc = "Field `SCUMUXIO136` reader - SCU_MUX_IO136"]
pub type Scumuxio136R = crate::FieldReader;
#[doc = "Field `SCUMUXIO136` writer - SCU_MUX_IO136"]
pub type Scumuxio136W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO137` reader - SCU_MUX_IO137"]
pub type Scumuxio137R = crate::FieldReader;
#[doc = "Field `SCUMUXIO137` writer - SCU_MUX_IO137"]
pub type Scumuxio137W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO138` reader - SCU_MUX_IO138"]
pub type Scumuxio138R = crate::FieldReader;
#[doc = "Field `SCUMUXIO138` writer - SCU_MUX_IO138"]
pub type Scumuxio138W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO139` reader - SCU_MUX_IO139"]
pub type Scumuxio139R = crate::FieldReader;
#[doc = "Field `SCUMUXIO139` writer - SCU_MUX_IO139"]
pub type Scumuxio139W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO140` reader - SCU_MUX_IO140"]
pub type Scumuxio140R = crate::FieldReader;
#[doc = "Field `SCUMUXIO140` writer - SCU_MUX_IO140"]
pub type Scumuxio140W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO141` reader - SCU_MUX_IO141"]
pub type Scumuxio141R = crate::FieldReader;
#[doc = "Field `SCUMUXIO141` writer - SCU_MUX_IO141"]
pub type Scumuxio141W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO142` reader - SCU_MUX_IO142"]
pub type Scumuxio142R = crate::FieldReader;
#[doc = "Field `SCUMUXIO142` writer - SCU_MUX_IO142"]
pub type Scumuxio142W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO143` reader - SCU_MUX_IO143"]
pub type Scumuxio143R = crate::FieldReader;
#[doc = "Field `SCUMUXIO143` writer - SCU_MUX_IO143"]
pub type Scumuxio143W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO136"]
    #[inline(always)]
    pub fn scumuxio136(&self) -> Scumuxio136R {
        Scumuxio136R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO137"]
    #[inline(always)]
    pub fn scumuxio137(&self) -> Scumuxio137R {
        Scumuxio137R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO138"]
    #[inline(always)]
    pub fn scumuxio138(&self) -> Scumuxio138R {
        Scumuxio138R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO139"]
    #[inline(always)]
    pub fn scumuxio139(&self) -> Scumuxio139R {
        Scumuxio139R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO140"]
    #[inline(always)]
    pub fn scumuxio140(&self) -> Scumuxio140R {
        Scumuxio140R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO141"]
    #[inline(always)]
    pub fn scumuxio141(&self) -> Scumuxio141R {
        Scumuxio141R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO142"]
    #[inline(always)]
    pub fn scumuxio142(&self) -> Scumuxio142R {
        Scumuxio142R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO143"]
    #[inline(always)]
    pub fn scumuxio143(&self) -> Scumuxio143R {
        Scumuxio143R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO136"]
    #[inline(always)]
    pub fn scumuxio136(&mut self) -> Scumuxio136W<Scu444Spec> {
        Scumuxio136W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO137"]
    #[inline(always)]
    pub fn scumuxio137(&mut self) -> Scumuxio137W<Scu444Spec> {
        Scumuxio137W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO138"]
    #[inline(always)]
    pub fn scumuxio138(&mut self) -> Scumuxio138W<Scu444Spec> {
        Scumuxio138W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO139"]
    #[inline(always)]
    pub fn scumuxio139(&mut self) -> Scumuxio139W<Scu444Spec> {
        Scumuxio139W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO140"]
    #[inline(always)]
    pub fn scumuxio140(&mut self) -> Scumuxio140W<Scu444Spec> {
        Scumuxio140W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO141"]
    #[inline(always)]
    pub fn scumuxio141(&mut self) -> Scumuxio141W<Scu444Spec> {
        Scumuxio141W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO142"]
    #[inline(always)]
    pub fn scumuxio142(&mut self) -> Scumuxio142W<Scu444Spec> {
        Scumuxio142W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO143"]
    #[inline(always)]
    pub fn scumuxio143(&mut self) -> Scumuxio143W<Scu444Spec> {
        Scumuxio143W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`scu444::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu444::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu444Spec;
impl crate::RegisterSpec for Scu444Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu444::R`](R) reader structure"]
impl crate::Readable for Scu444Spec {}
#[doc = "`write(|w| ..)` method takes [`scu444::W`](W) writer structure"]
impl crate::Writable for Scu444Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU444 to value 0x0222"]
impl crate::Resettable for Scu444Spec {
    const RESET_VALUE: u32 = 0x0222;
}
