#[doc = "Register `SCU45C` reader"]
pub type R = crate::R<Scu45cSpec>;
#[doc = "Register `SCU45C` writer"]
pub type W = crate::W<Scu45cSpec>;
#[doc = "Field `SCUMUXIO184` reader - SCU_MUX_IO184"]
pub type Scumuxio184R = crate::FieldReader;
#[doc = "Field `SCUMUXIO184` writer - SCU_MUX_IO184"]
pub type Scumuxio184W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO185` reader - SCU_MUX_IO185"]
pub type Scumuxio185R = crate::FieldReader;
#[doc = "Field `SCUMUXIO185` writer - SCU_MUX_IO185"]
pub type Scumuxio185W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO186` reader - SCU_MUX_IO186"]
pub type Scumuxio186R = crate::FieldReader;
#[doc = "Field `SCUMUXIO186` writer - SCU_MUX_IO186"]
pub type Scumuxio186W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO187` reader - SCU_MUX_IO187"]
pub type Scumuxio187R = crate::FieldReader;
#[doc = "Field `SCUMUXIO187` writer - SCU_MUX_IO187"]
pub type Scumuxio187W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO188` reader - SCU_MUX_IO188"]
pub type Scumuxio188R = crate::FieldReader;
#[doc = "Field `SCUMUXIO188` writer - SCU_MUX_IO188"]
pub type Scumuxio188W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO189` reader - SCU_MUX_IO189"]
pub type Scumuxio189R = crate::FieldReader;
#[doc = "Field `SCUMUXIO189` writer - SCU_MUX_IO189"]
pub type Scumuxio189W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO190` reader - SCU_MUX_IO190"]
pub type Scumuxio190R = crate::FieldReader;
#[doc = "Field `SCUMUXIO190` writer - SCU_MUX_IO190"]
pub type Scumuxio190W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO191` reader - SCU_MUX_IO191"]
pub type Scumuxio191R = crate::FieldReader;
#[doc = "Field `SCUMUXIO191` writer - SCU_MUX_IO191"]
pub type Scumuxio191W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO184"]
    #[inline(always)]
    pub fn scumuxio184(&self) -> Scumuxio184R {
        Scumuxio184R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO185"]
    #[inline(always)]
    pub fn scumuxio185(&self) -> Scumuxio185R {
        Scumuxio185R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO186"]
    #[inline(always)]
    pub fn scumuxio186(&self) -> Scumuxio186R {
        Scumuxio186R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO187"]
    #[inline(always)]
    pub fn scumuxio187(&self) -> Scumuxio187R {
        Scumuxio187R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO188"]
    #[inline(always)]
    pub fn scumuxio188(&self) -> Scumuxio188R {
        Scumuxio188R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO189"]
    #[inline(always)]
    pub fn scumuxio189(&self) -> Scumuxio189R {
        Scumuxio189R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO190"]
    #[inline(always)]
    pub fn scumuxio190(&self) -> Scumuxio190R {
        Scumuxio190R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO191"]
    #[inline(always)]
    pub fn scumuxio191(&self) -> Scumuxio191R {
        Scumuxio191R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO184"]
    #[inline(always)]
    pub fn scumuxio184(&mut self) -> Scumuxio184W<Scu45cSpec> {
        Scumuxio184W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO185"]
    #[inline(always)]
    pub fn scumuxio185(&mut self) -> Scumuxio185W<Scu45cSpec> {
        Scumuxio185W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO186"]
    #[inline(always)]
    pub fn scumuxio186(&mut self) -> Scumuxio186W<Scu45cSpec> {
        Scumuxio186W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO187"]
    #[inline(always)]
    pub fn scumuxio187(&mut self) -> Scumuxio187W<Scu45cSpec> {
        Scumuxio187W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO188"]
    #[inline(always)]
    pub fn scumuxio188(&mut self) -> Scumuxio188W<Scu45cSpec> {
        Scumuxio188W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO189"]
    #[inline(always)]
    pub fn scumuxio189(&mut self) -> Scumuxio189W<Scu45cSpec> {
        Scumuxio189W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO190"]
    #[inline(always)]
    pub fn scumuxio190(&mut self) -> Scumuxio190W<Scu45cSpec> {
        Scumuxio190W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO191"]
    #[inline(always)]
    pub fn scumuxio191(&mut self) -> Scumuxio191W<Scu45cSpec> {
        Scumuxio191W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`scu45c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu45c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu45cSpec;
impl crate::RegisterSpec for Scu45cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu45c::R`](R) reader structure"]
impl crate::Readable for Scu45cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu45c::W`](W) writer structure"]
impl crate::Writable for Scu45cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU45C to value 0x1001_1111"]
impl crate::Resettable for Scu45cSpec {
    const RESET_VALUE: u32 = 0x1001_1111;
}
