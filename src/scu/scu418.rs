#[doc = "Register `SCU418` reader"]
pub type R = crate::R<Scu418Spec>;
#[doc = "Register `SCU418` writer"]
pub type W = crate::W<Scu418Spec>;
#[doc = "Field `SCUMUXIO048` reader - SCU_MUX_IO048"]
pub type Scumuxio048R = crate::FieldReader;
#[doc = "Field `SCUMUXIO048` writer - SCU_MUX_IO048"]
pub type Scumuxio048W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO049` reader - SCU_MUX_IO049"]
pub type Scumuxio049R = crate::FieldReader;
#[doc = "Field `SCUMUXIO049` writer - SCU_MUX_IO049"]
pub type Scumuxio049W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO050` reader - SCU_MUX_IO050"]
pub type Scumuxio050R = crate::FieldReader;
#[doc = "Field `SCUMUXIO050` writer - SCU_MUX_IO050"]
pub type Scumuxio050W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO051` reader - SCU_MUX_IO051"]
pub type Scumuxio051R = crate::FieldReader;
#[doc = "Field `SCUMUXIO051` writer - SCU_MUX_IO051"]
pub type Scumuxio051W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO052` reader - SCU_MUX_IO052"]
pub type Scumuxio052R = crate::FieldReader;
#[doc = "Field `SCUMUXIO052` writer - SCU_MUX_IO052"]
pub type Scumuxio052W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO053` reader - SCU_MUX_IO053"]
pub type Scumuxio053R = crate::FieldReader;
#[doc = "Field `SCUMUXIO053` writer - SCU_MUX_IO053"]
pub type Scumuxio053W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO054` reader - SCU_MUX_IO054"]
pub type Scumuxio054R = crate::FieldReader;
#[doc = "Field `SCUMUXIO054` writer - SCU_MUX_IO054"]
pub type Scumuxio054W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO055` reader - SCU_MUX_IO055"]
pub type Scumuxio055R = crate::FieldReader;
#[doc = "Field `SCUMUXIO055` writer - SCU_MUX_IO055"]
pub type Scumuxio055W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO048"]
    #[inline(always)]
    pub fn scumuxio048(&self) -> Scumuxio048R {
        Scumuxio048R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO049"]
    #[inline(always)]
    pub fn scumuxio049(&self) -> Scumuxio049R {
        Scumuxio049R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO050"]
    #[inline(always)]
    pub fn scumuxio050(&self) -> Scumuxio050R {
        Scumuxio050R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO051"]
    #[inline(always)]
    pub fn scumuxio051(&self) -> Scumuxio051R {
        Scumuxio051R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO052"]
    #[inline(always)]
    pub fn scumuxio052(&self) -> Scumuxio052R {
        Scumuxio052R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO053"]
    #[inline(always)]
    pub fn scumuxio053(&self) -> Scumuxio053R {
        Scumuxio053R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO054"]
    #[inline(always)]
    pub fn scumuxio054(&self) -> Scumuxio054R {
        Scumuxio054R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO055"]
    #[inline(always)]
    pub fn scumuxio055(&self) -> Scumuxio055R {
        Scumuxio055R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO048"]
    #[inline(always)]
    pub fn scumuxio048(&mut self) -> Scumuxio048W<Scu418Spec> {
        Scumuxio048W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO049"]
    #[inline(always)]
    pub fn scumuxio049(&mut self) -> Scumuxio049W<Scu418Spec> {
        Scumuxio049W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO050"]
    #[inline(always)]
    pub fn scumuxio050(&mut self) -> Scumuxio050W<Scu418Spec> {
        Scumuxio050W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO051"]
    #[inline(always)]
    pub fn scumuxio051(&mut self) -> Scumuxio051W<Scu418Spec> {
        Scumuxio051W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO052"]
    #[inline(always)]
    pub fn scumuxio052(&mut self) -> Scumuxio052W<Scu418Spec> {
        Scumuxio052W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO053"]
    #[inline(always)]
    pub fn scumuxio053(&mut self) -> Scumuxio053W<Scu418Spec> {
        Scumuxio053W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO054"]
    #[inline(always)]
    pub fn scumuxio054(&mut self) -> Scumuxio054W<Scu418Spec> {
        Scumuxio054W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO055"]
    #[inline(always)]
    pub fn scumuxio055(&mut self) -> Scumuxio055W<Scu418Spec> {
        Scumuxio055W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu418::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu418::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu418Spec;
impl crate::RegisterSpec for Scu418Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu418::R`](R) reader structure"]
impl crate::Readable for Scu418Spec {}
#[doc = "`write(|w| ..)` method takes [`scu418::W`](W) writer structure"]
impl crate::Writable for Scu418Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU418 to value 0"]
impl crate::Resettable for Scu418Spec {}
