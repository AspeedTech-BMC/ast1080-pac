#[doc = "Register `SCU42C` reader"]
pub type R = crate::R<Scu42cSpec>;
#[doc = "Register `SCU42C` writer"]
pub type W = crate::W<Scu42cSpec>;
#[doc = "Field `SCUMUXIO088` reader - SCU_MUX_IO088"]
pub type Scumuxio088R = crate::FieldReader;
#[doc = "Field `SCUMUXIO088` writer - SCU_MUX_IO088"]
pub type Scumuxio088W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO089` reader - SCU_MUX_IO089"]
pub type Scumuxio089R = crate::FieldReader;
#[doc = "Field `SCUMUXIO089` writer - SCU_MUX_IO089"]
pub type Scumuxio089W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO090` reader - SCU_MUX_IO090"]
pub type Scumuxio090R = crate::FieldReader;
#[doc = "Field `SCUMUXIO090` writer - SCU_MUX_IO090"]
pub type Scumuxio090W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO091` reader - SCU_MUX_IO091"]
pub type Scumuxio091R = crate::FieldReader;
#[doc = "Field `SCUMUXIO091` writer - SCU_MUX_IO091"]
pub type Scumuxio091W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO092` reader - SCU_MUX_IO092"]
pub type Scumuxio092R = crate::FieldReader;
#[doc = "Field `SCUMUXIO092` writer - SCU_MUX_IO092"]
pub type Scumuxio092W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO093` reader - SCU_MUX_IO093"]
pub type Scumuxio093R = crate::FieldReader;
#[doc = "Field `SCUMUXIO093` writer - SCU_MUX_IO093"]
pub type Scumuxio093W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO094` reader - SCU_MUX_IO094"]
pub type Scumuxio094R = crate::FieldReader;
#[doc = "Field `SCUMUXIO094` writer - SCU_MUX_IO094"]
pub type Scumuxio094W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO095` reader - SCU_MUX_IO095"]
pub type Scumuxio095R = crate::FieldReader;
#[doc = "Field `SCUMUXIO095` writer - SCU_MUX_IO095"]
pub type Scumuxio095W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO088"]
    #[inline(always)]
    pub fn scumuxio088(&self) -> Scumuxio088R {
        Scumuxio088R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO089"]
    #[inline(always)]
    pub fn scumuxio089(&self) -> Scumuxio089R {
        Scumuxio089R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO090"]
    #[inline(always)]
    pub fn scumuxio090(&self) -> Scumuxio090R {
        Scumuxio090R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO091"]
    #[inline(always)]
    pub fn scumuxio091(&self) -> Scumuxio091R {
        Scumuxio091R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO092"]
    #[inline(always)]
    pub fn scumuxio092(&self) -> Scumuxio092R {
        Scumuxio092R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO093"]
    #[inline(always)]
    pub fn scumuxio093(&self) -> Scumuxio093R {
        Scumuxio093R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO094"]
    #[inline(always)]
    pub fn scumuxio094(&self) -> Scumuxio094R {
        Scumuxio094R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO095"]
    #[inline(always)]
    pub fn scumuxio095(&self) -> Scumuxio095R {
        Scumuxio095R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO088"]
    #[inline(always)]
    pub fn scumuxio088(&mut self) -> Scumuxio088W<Scu42cSpec> {
        Scumuxio088W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO089"]
    #[inline(always)]
    pub fn scumuxio089(&mut self) -> Scumuxio089W<Scu42cSpec> {
        Scumuxio089W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO090"]
    #[inline(always)]
    pub fn scumuxio090(&mut self) -> Scumuxio090W<Scu42cSpec> {
        Scumuxio090W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO091"]
    #[inline(always)]
    pub fn scumuxio091(&mut self) -> Scumuxio091W<Scu42cSpec> {
        Scumuxio091W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO092"]
    #[inline(always)]
    pub fn scumuxio092(&mut self) -> Scumuxio092W<Scu42cSpec> {
        Scumuxio092W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO093"]
    #[inline(always)]
    pub fn scumuxio093(&mut self) -> Scumuxio093W<Scu42cSpec> {
        Scumuxio093W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO094"]
    #[inline(always)]
    pub fn scumuxio094(&mut self) -> Scumuxio094W<Scu42cSpec> {
        Scumuxio094W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO095"]
    #[inline(always)]
    pub fn scumuxio095(&mut self) -> Scumuxio095W<Scu42cSpec> {
        Scumuxio095W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`scu42c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu42c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu42cSpec;
impl crate::RegisterSpec for Scu42cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu42c::R`](R) reader structure"]
impl crate::Readable for Scu42cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu42c::W`](W) writer structure"]
impl crate::Writable for Scu42cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU42C to value 0"]
impl crate::Resettable for Scu42cSpec {}
