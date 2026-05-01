#[doc = "Register `SCU438` reader"]
pub type R = crate::R<Scu438Spec>;
#[doc = "Register `SCU438` writer"]
pub type W = crate::W<Scu438Spec>;
#[doc = "Field `SCUMUXIO112` reader - SCU_MUX_IO112"]
pub type Scumuxio112R = crate::FieldReader;
#[doc = "Field `SCUMUXIO112` writer - SCU_MUX_IO112"]
pub type Scumuxio112W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO113` reader - SCU_MUX_IO113"]
pub type Scumuxio113R = crate::FieldReader;
#[doc = "Field `SCUMUXIO113` writer - SCU_MUX_IO113"]
pub type Scumuxio113W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO114` reader - SCU_MUX_IO114"]
pub type Scumuxio114R = crate::FieldReader;
#[doc = "Field `SCUMUXIO114` writer - SCU_MUX_IO114"]
pub type Scumuxio114W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO115` reader - SCU_MUX_IO115"]
pub type Scumuxio115R = crate::FieldReader;
#[doc = "Field `SCUMUXIO115` writer - SCU_MUX_IO115"]
pub type Scumuxio115W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO116` reader - SCU_MUX_IO116"]
pub type Scumuxio116R = crate::FieldReader;
#[doc = "Field `SCUMUXIO116` writer - SCU_MUX_IO116"]
pub type Scumuxio116W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO117` reader - SCU_MUX_IO117"]
pub type Scumuxio117R = crate::FieldReader;
#[doc = "Field `SCUMUXIO117` writer - SCU_MUX_IO117"]
pub type Scumuxio117W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO118` reader - SCU_MUX_IO118"]
pub type Scumuxio118R = crate::FieldReader;
#[doc = "Field `SCUMUXIO118` writer - SCU_MUX_IO118"]
pub type Scumuxio118W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO119` reader - SCU_MUX_IO119"]
pub type Scumuxio119R = crate::FieldReader;
#[doc = "Field `SCUMUXIO119` writer - SCU_MUX_IO119"]
pub type Scumuxio119W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO112"]
    #[inline(always)]
    pub fn scumuxio112(&self) -> Scumuxio112R {
        Scumuxio112R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO113"]
    #[inline(always)]
    pub fn scumuxio113(&self) -> Scumuxio113R {
        Scumuxio113R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO114"]
    #[inline(always)]
    pub fn scumuxio114(&self) -> Scumuxio114R {
        Scumuxio114R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO115"]
    #[inline(always)]
    pub fn scumuxio115(&self) -> Scumuxio115R {
        Scumuxio115R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO116"]
    #[inline(always)]
    pub fn scumuxio116(&self) -> Scumuxio116R {
        Scumuxio116R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO117"]
    #[inline(always)]
    pub fn scumuxio117(&self) -> Scumuxio117R {
        Scumuxio117R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO118"]
    #[inline(always)]
    pub fn scumuxio118(&self) -> Scumuxio118R {
        Scumuxio118R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO119"]
    #[inline(always)]
    pub fn scumuxio119(&self) -> Scumuxio119R {
        Scumuxio119R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO112"]
    #[inline(always)]
    pub fn scumuxio112(&mut self) -> Scumuxio112W<Scu438Spec> {
        Scumuxio112W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO113"]
    #[inline(always)]
    pub fn scumuxio113(&mut self) -> Scumuxio113W<Scu438Spec> {
        Scumuxio113W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO114"]
    #[inline(always)]
    pub fn scumuxio114(&mut self) -> Scumuxio114W<Scu438Spec> {
        Scumuxio114W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO115"]
    #[inline(always)]
    pub fn scumuxio115(&mut self) -> Scumuxio115W<Scu438Spec> {
        Scumuxio115W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO116"]
    #[inline(always)]
    pub fn scumuxio116(&mut self) -> Scumuxio116W<Scu438Spec> {
        Scumuxio116W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO117"]
    #[inline(always)]
    pub fn scumuxio117(&mut self) -> Scumuxio117W<Scu438Spec> {
        Scumuxio117W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO118"]
    #[inline(always)]
    pub fn scumuxio118(&mut self) -> Scumuxio118W<Scu438Spec> {
        Scumuxio118W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO119"]
    #[inline(always)]
    pub fn scumuxio119(&mut self) -> Scumuxio119W<Scu438Spec> {
        Scumuxio119W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`scu438::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu438::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu438Spec;
impl crate::RegisterSpec for Scu438Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu438::R`](R) reader structure"]
impl crate::Readable for Scu438Spec {}
#[doc = "`write(|w| ..)` method takes [`scu438::W`](W) writer structure"]
impl crate::Writable for Scu438Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU438 to value 0"]
impl crate::Resettable for Scu438Spec {}
