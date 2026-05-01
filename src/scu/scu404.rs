#[doc = "Register `SCU404` reader"]
pub type R = crate::R<Scu404Spec>;
#[doc = "Register `SCU404` writer"]
pub type W = crate::W<Scu404Spec>;
#[doc = "Field `SCUMUXIO008` reader - SCU_MUX_IO008"]
pub type Scumuxio008R = crate::FieldReader;
#[doc = "Field `SCUMUXIO008` writer - SCU_MUX_IO008"]
pub type Scumuxio008W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO009` reader - SCU_MUX_IO009"]
pub type Scumuxio009R = crate::FieldReader;
#[doc = "Field `SCUMUXIO009` writer - SCU_MUX_IO009"]
pub type Scumuxio009W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO010` reader - SCU_MUX_IO010"]
pub type Scumuxio010R = crate::FieldReader;
#[doc = "Field `SCUMUXIO010` writer - SCU_MUX_IO010"]
pub type Scumuxio010W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO011` reader - SCU_MUX_IO011"]
pub type Scumuxio011R = crate::FieldReader;
#[doc = "Field `SCUMUXIO011` writer - SCU_MUX_IO011"]
pub type Scumuxio011W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO012` reader - SCU_MUX_IO012"]
pub type Scumuxio012R = crate::FieldReader;
#[doc = "Field `SCUMUXIO012` writer - SCU_MUX_IO012"]
pub type Scumuxio012W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO013` reader - SCU_MUX_IO013"]
pub type Scumuxio013R = crate::FieldReader;
#[doc = "Field `SCUMUXIO013` writer - SCU_MUX_IO013"]
pub type Scumuxio013W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO014` reader - SCU_MUX_IO014"]
pub type Scumuxio014R = crate::FieldReader;
#[doc = "Field `SCUMUXIO014` writer - SCU_MUX_IO014"]
pub type Scumuxio014W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO015` reader - SCU_MUX_IO015"]
pub type Scumuxio015R = crate::FieldReader;
#[doc = "Field `SCUMUXIO015` writer - SCU_MUX_IO015"]
pub type Scumuxio015W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO008"]
    #[inline(always)]
    pub fn scumuxio008(&self) -> Scumuxio008R {
        Scumuxio008R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO009"]
    #[inline(always)]
    pub fn scumuxio009(&self) -> Scumuxio009R {
        Scumuxio009R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO010"]
    #[inline(always)]
    pub fn scumuxio010(&self) -> Scumuxio010R {
        Scumuxio010R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO011"]
    #[inline(always)]
    pub fn scumuxio011(&self) -> Scumuxio011R {
        Scumuxio011R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO012"]
    #[inline(always)]
    pub fn scumuxio012(&self) -> Scumuxio012R {
        Scumuxio012R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO013"]
    #[inline(always)]
    pub fn scumuxio013(&self) -> Scumuxio013R {
        Scumuxio013R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO014"]
    #[inline(always)]
    pub fn scumuxio014(&self) -> Scumuxio014R {
        Scumuxio014R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO015"]
    #[inline(always)]
    pub fn scumuxio015(&self) -> Scumuxio015R {
        Scumuxio015R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO008"]
    #[inline(always)]
    pub fn scumuxio008(&mut self) -> Scumuxio008W<Scu404Spec> {
        Scumuxio008W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO009"]
    #[inline(always)]
    pub fn scumuxio009(&mut self) -> Scumuxio009W<Scu404Spec> {
        Scumuxio009W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO010"]
    #[inline(always)]
    pub fn scumuxio010(&mut self) -> Scumuxio010W<Scu404Spec> {
        Scumuxio010W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO011"]
    #[inline(always)]
    pub fn scumuxio011(&mut self) -> Scumuxio011W<Scu404Spec> {
        Scumuxio011W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO012"]
    #[inline(always)]
    pub fn scumuxio012(&mut self) -> Scumuxio012W<Scu404Spec> {
        Scumuxio012W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO013"]
    #[inline(always)]
    pub fn scumuxio013(&mut self) -> Scumuxio013W<Scu404Spec> {
        Scumuxio013W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO014"]
    #[inline(always)]
    pub fn scumuxio014(&mut self) -> Scumuxio014W<Scu404Spec> {
        Scumuxio014W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO015"]
    #[inline(always)]
    pub fn scumuxio015(&mut self) -> Scumuxio015W<Scu404Spec> {
        Scumuxio015W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu404::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu404::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu404Spec;
impl crate::RegisterSpec for Scu404Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu404::R`](R) reader structure"]
impl crate::Readable for Scu404Spec {}
#[doc = "`write(|w| ..)` method takes [`scu404::W`](W) writer structure"]
impl crate::Writable for Scu404Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU404 to value 0"]
impl crate::Resettable for Scu404Spec {}
