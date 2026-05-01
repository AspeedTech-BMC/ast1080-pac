#[doc = "Register `SCU400` reader"]
pub type R = crate::R<Scu400Spec>;
#[doc = "Register `SCU400` writer"]
pub type W = crate::W<Scu400Spec>;
#[doc = "Field `SCUMUXIO000` reader - SCU_MUX_IO000"]
pub type Scumuxio000R = crate::FieldReader;
#[doc = "Field `SCUMUXIO000` writer - SCU_MUX_IO000"]
pub type Scumuxio000W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO001` reader - SCU_MUX_IO001"]
pub type Scumuxio001R = crate::FieldReader;
#[doc = "Field `SCUMUXIO001` writer - SCU_MUX_IO001"]
pub type Scumuxio001W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO002` reader - SCU_MUX_IO002"]
pub type Scumuxio002R = crate::FieldReader;
#[doc = "Field `SCUMUXIO002` writer - SCU_MUX_IO002"]
pub type Scumuxio002W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO003` reader - SCU_MUX_IO003"]
pub type Scumuxio003R = crate::FieldReader;
#[doc = "Field `SCUMUXIO003` writer - SCU_MUX_IO003"]
pub type Scumuxio003W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO004` reader - SCU_MUX_IO004"]
pub type Scumuxio004R = crate::FieldReader;
#[doc = "Field `SCUMUXIO004` writer - SCU_MUX_IO004"]
pub type Scumuxio004W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO005` reader - SCU_MUX_IO005"]
pub type Scumuxio005R = crate::FieldReader;
#[doc = "Field `SCUMUXIO005` writer - SCU_MUX_IO005"]
pub type Scumuxio005W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO006` reader - SCU_MUX_IO006"]
pub type Scumuxio006R = crate::FieldReader;
#[doc = "Field `SCUMUXIO006` writer - SCU_MUX_IO006"]
pub type Scumuxio006W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO007` reader - SCU_MUX_IO007"]
pub type Scumuxio007R = crate::FieldReader;
#[doc = "Field `SCUMUXIO007` writer - SCU_MUX_IO007"]
pub type Scumuxio007W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO000"]
    #[inline(always)]
    pub fn scumuxio000(&self) -> Scumuxio000R {
        Scumuxio000R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO001"]
    #[inline(always)]
    pub fn scumuxio001(&self) -> Scumuxio001R {
        Scumuxio001R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO002"]
    #[inline(always)]
    pub fn scumuxio002(&self) -> Scumuxio002R {
        Scumuxio002R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO003"]
    #[inline(always)]
    pub fn scumuxio003(&self) -> Scumuxio003R {
        Scumuxio003R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO004"]
    #[inline(always)]
    pub fn scumuxio004(&self) -> Scumuxio004R {
        Scumuxio004R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO005"]
    #[inline(always)]
    pub fn scumuxio005(&self) -> Scumuxio005R {
        Scumuxio005R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO006"]
    #[inline(always)]
    pub fn scumuxio006(&self) -> Scumuxio006R {
        Scumuxio006R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO007"]
    #[inline(always)]
    pub fn scumuxio007(&self) -> Scumuxio007R {
        Scumuxio007R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO000"]
    #[inline(always)]
    pub fn scumuxio000(&mut self) -> Scumuxio000W<Scu400Spec> {
        Scumuxio000W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO001"]
    #[inline(always)]
    pub fn scumuxio001(&mut self) -> Scumuxio001W<Scu400Spec> {
        Scumuxio001W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO002"]
    #[inline(always)]
    pub fn scumuxio002(&mut self) -> Scumuxio002W<Scu400Spec> {
        Scumuxio002W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO003"]
    #[inline(always)]
    pub fn scumuxio003(&mut self) -> Scumuxio003W<Scu400Spec> {
        Scumuxio003W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO004"]
    #[inline(always)]
    pub fn scumuxio004(&mut self) -> Scumuxio004W<Scu400Spec> {
        Scumuxio004W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO005"]
    #[inline(always)]
    pub fn scumuxio005(&mut self) -> Scumuxio005W<Scu400Spec> {
        Scumuxio005W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO006"]
    #[inline(always)]
    pub fn scumuxio006(&mut self) -> Scumuxio006W<Scu400Spec> {
        Scumuxio006W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO007"]
    #[inline(always)]
    pub fn scumuxio007(&mut self) -> Scumuxio007W<Scu400Spec> {
        Scumuxio007W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu400::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu400::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu400Spec;
impl crate::RegisterSpec for Scu400Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu400::R`](R) reader structure"]
impl crate::Readable for Scu400Spec {}
#[doc = "`write(|w| ..)` method takes [`scu400::W`](W) writer structure"]
impl crate::Writable for Scu400Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU400 to value 0"]
impl crate::Resettable for Scu400Spec {}
