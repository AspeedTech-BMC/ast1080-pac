#[doc = "Register `SCU43C` reader"]
pub type R = crate::R<Scu43cSpec>;
#[doc = "Register `SCU43C` writer"]
pub type W = crate::W<Scu43cSpec>;
#[doc = "Field `SCUMUXIO120` reader - SCU_MUX_IO120"]
pub type Scumuxio120R = crate::FieldReader;
#[doc = "Field `SCUMUXIO120` writer - SCU_MUX_IO120"]
pub type Scumuxio120W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO121` reader - SCU_MUX_IO121"]
pub type Scumuxio121R = crate::FieldReader;
#[doc = "Field `SCUMUXIO121` writer - SCU_MUX_IO121"]
pub type Scumuxio121W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO122` reader - SCU_MUX_IO122"]
pub type Scumuxio122R = crate::FieldReader;
#[doc = "Field `SCUMUXIO122` writer - SCU_MUX_IO122"]
pub type Scumuxio122W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO123` reader - SCU_MUX_IO123"]
pub type Scumuxio123R = crate::FieldReader;
#[doc = "Field `SCUMUXIO123` writer - SCU_MUX_IO123"]
pub type Scumuxio123W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO124` reader - SCU_MUX_IO124"]
pub type Scumuxio124R = crate::FieldReader;
#[doc = "Field `SCUMUXIO124` writer - SCU_MUX_IO124"]
pub type Scumuxio124W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO125` reader - SCU_MUX_IO125"]
pub type Scumuxio125R = crate::FieldReader;
#[doc = "Field `SCUMUXIO125` writer - SCU_MUX_IO125"]
pub type Scumuxio125W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO126` reader - SCU_MUX_IO126"]
pub type Scumuxio126R = crate::FieldReader;
#[doc = "Field `SCUMUXIO126` writer - SCU_MUX_IO126"]
pub type Scumuxio126W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO127` reader - SCU_MUX_IO127"]
pub type Scumuxio127R = crate::FieldReader;
#[doc = "Field `SCUMUXIO127` writer - SCU_MUX_IO127"]
pub type Scumuxio127W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO120"]
    #[inline(always)]
    pub fn scumuxio120(&self) -> Scumuxio120R {
        Scumuxio120R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO121"]
    #[inline(always)]
    pub fn scumuxio121(&self) -> Scumuxio121R {
        Scumuxio121R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO122"]
    #[inline(always)]
    pub fn scumuxio122(&self) -> Scumuxio122R {
        Scumuxio122R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO123"]
    #[inline(always)]
    pub fn scumuxio123(&self) -> Scumuxio123R {
        Scumuxio123R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO124"]
    #[inline(always)]
    pub fn scumuxio124(&self) -> Scumuxio124R {
        Scumuxio124R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO125"]
    #[inline(always)]
    pub fn scumuxio125(&self) -> Scumuxio125R {
        Scumuxio125R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO126"]
    #[inline(always)]
    pub fn scumuxio126(&self) -> Scumuxio126R {
        Scumuxio126R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO127"]
    #[inline(always)]
    pub fn scumuxio127(&self) -> Scumuxio127R {
        Scumuxio127R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO120"]
    #[inline(always)]
    pub fn scumuxio120(&mut self) -> Scumuxio120W<Scu43cSpec> {
        Scumuxio120W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO121"]
    #[inline(always)]
    pub fn scumuxio121(&mut self) -> Scumuxio121W<Scu43cSpec> {
        Scumuxio121W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO122"]
    #[inline(always)]
    pub fn scumuxio122(&mut self) -> Scumuxio122W<Scu43cSpec> {
        Scumuxio122W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO123"]
    #[inline(always)]
    pub fn scumuxio123(&mut self) -> Scumuxio123W<Scu43cSpec> {
        Scumuxio123W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO124"]
    #[inline(always)]
    pub fn scumuxio124(&mut self) -> Scumuxio124W<Scu43cSpec> {
        Scumuxio124W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO125"]
    #[inline(always)]
    pub fn scumuxio125(&mut self) -> Scumuxio125W<Scu43cSpec> {
        Scumuxio125W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO126"]
    #[inline(always)]
    pub fn scumuxio126(&mut self) -> Scumuxio126W<Scu43cSpec> {
        Scumuxio126W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO127"]
    #[inline(always)]
    pub fn scumuxio127(&mut self) -> Scumuxio127W<Scu43cSpec> {
        Scumuxio127W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`scu43c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu43c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu43cSpec;
impl crate::RegisterSpec for Scu43cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu43c::R`](R) reader structure"]
impl crate::Readable for Scu43cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu43c::W`](W) writer structure"]
impl crate::Writable for Scu43cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU43C to value 0x1100_0000"]
impl crate::Resettable for Scu43cSpec {
    const RESET_VALUE: u32 = 0x1100_0000;
}
