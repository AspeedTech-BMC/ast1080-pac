#[doc = "Register `SCU41C` reader"]
pub type R = crate::R<Scu41cSpec>;
#[doc = "Register `SCU41C` writer"]
pub type W = crate::W<Scu41cSpec>;
#[doc = "Field `SCUMUXIO056` reader - SCU_MUX_IO056"]
pub type Scumuxio056R = crate::FieldReader;
#[doc = "Field `SCUMUXIO056` writer - SCU_MUX_IO056"]
pub type Scumuxio056W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO057` reader - SCU_MUX_IO057"]
pub type Scumuxio057R = crate::FieldReader;
#[doc = "Field `SCUMUXIO057` writer - SCU_MUX_IO057"]
pub type Scumuxio057W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO058` reader - SCU_MUX_IO058"]
pub type Scumuxio058R = crate::FieldReader;
#[doc = "Field `SCUMUXIO058` writer - SCU_MUX_IO058"]
pub type Scumuxio058W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO059` reader - SCU_MUX_IO059"]
pub type Scumuxio059R = crate::FieldReader;
#[doc = "Field `SCUMUXIO059` writer - SCU_MUX_IO059"]
pub type Scumuxio059W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO060` reader - SCU_MUX_IO060"]
pub type Scumuxio060R = crate::FieldReader;
#[doc = "Field `SCUMUXIO060` writer - SCU_MUX_IO060"]
pub type Scumuxio060W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO061` reader - SCU_MUX_IO061"]
pub type Scumuxio061R = crate::FieldReader;
#[doc = "Field `SCUMUXIO061` writer - SCU_MUX_IO061"]
pub type Scumuxio061W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO062` reader - SCU_MUX_IO062"]
pub type Scumuxio062R = crate::FieldReader;
#[doc = "Field `SCUMUXIO062` writer - SCU_MUX_IO062"]
pub type Scumuxio062W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO063` reader - SCU_MUX_IO063"]
pub type Scumuxio063R = crate::FieldReader;
#[doc = "Field `SCUMUXIO063` writer - SCU_MUX_IO063"]
pub type Scumuxio063W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO056"]
    #[inline(always)]
    pub fn scumuxio056(&self) -> Scumuxio056R {
        Scumuxio056R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO057"]
    #[inline(always)]
    pub fn scumuxio057(&self) -> Scumuxio057R {
        Scumuxio057R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO058"]
    #[inline(always)]
    pub fn scumuxio058(&self) -> Scumuxio058R {
        Scumuxio058R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO059"]
    #[inline(always)]
    pub fn scumuxio059(&self) -> Scumuxio059R {
        Scumuxio059R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO060"]
    #[inline(always)]
    pub fn scumuxio060(&self) -> Scumuxio060R {
        Scumuxio060R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO061"]
    #[inline(always)]
    pub fn scumuxio061(&self) -> Scumuxio061R {
        Scumuxio061R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO062"]
    #[inline(always)]
    pub fn scumuxio062(&self) -> Scumuxio062R {
        Scumuxio062R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO063"]
    #[inline(always)]
    pub fn scumuxio063(&self) -> Scumuxio063R {
        Scumuxio063R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO056"]
    #[inline(always)]
    pub fn scumuxio056(&mut self) -> Scumuxio056W<Scu41cSpec> {
        Scumuxio056W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO057"]
    #[inline(always)]
    pub fn scumuxio057(&mut self) -> Scumuxio057W<Scu41cSpec> {
        Scumuxio057W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO058"]
    #[inline(always)]
    pub fn scumuxio058(&mut self) -> Scumuxio058W<Scu41cSpec> {
        Scumuxio058W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO059"]
    #[inline(always)]
    pub fn scumuxio059(&mut self) -> Scumuxio059W<Scu41cSpec> {
        Scumuxio059W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO060"]
    #[inline(always)]
    pub fn scumuxio060(&mut self) -> Scumuxio060W<Scu41cSpec> {
        Scumuxio060W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO061"]
    #[inline(always)]
    pub fn scumuxio061(&mut self) -> Scumuxio061W<Scu41cSpec> {
        Scumuxio061W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO062"]
    #[inline(always)]
    pub fn scumuxio062(&mut self) -> Scumuxio062W<Scu41cSpec> {
        Scumuxio062W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO063"]
    #[inline(always)]
    pub fn scumuxio063(&mut self) -> Scumuxio063W<Scu41cSpec> {
        Scumuxio063W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`scu41c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu41c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu41cSpec;
impl crate::RegisterSpec for Scu41cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu41c::R`](R) reader structure"]
impl crate::Readable for Scu41cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu41c::W`](W) writer structure"]
impl crate::Writable for Scu41cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU41C to value 0"]
impl crate::Resettable for Scu41cSpec {}
