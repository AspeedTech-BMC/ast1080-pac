#[doc = "Register `SCUF3C` reader"]
pub type R = crate::R<Scuf3cSpec>;
#[doc = "Register `SCUF3C` writer"]
pub type W = crate::W<Scuf3cSpec>;
#[doc = "Field `SCUREGRST780` reader - SCU_REG_RST_780"]
pub type Scuregrst780R = crate::BitReader;
#[doc = "Field `SCUREGRST780` writer - SCU_REG_RST_780"]
pub type Scuregrst780W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST784` reader - SCU_REG_RST_784"]
pub type Scuregrst784R = crate::BitReader;
#[doc = "Field `SCUREGRST784` writer - SCU_REG_RST_784"]
pub type Scuregrst784W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST788` reader - SCU_REG_RST_788"]
pub type Scuregrst788R = crate::BitReader;
#[doc = "Field `SCUREGRST788` writer - SCU_REG_RST_788"]
pub type Scuregrst788W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST78C` reader - SCU_REG_RST_78C"]
pub type Scuregrst78cR = crate::BitReader;
#[doc = "Field `SCUREGRST78C` writer - SCU_REG_RST_78C"]
pub type Scuregrst78cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST790` reader - SCU_REG_RST_790"]
pub type Scuregrst790R = crate::BitReader;
#[doc = "Field `SCUREGRST790` writer - SCU_REG_RST_790"]
pub type Scuregrst790W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST794` reader - SCU_REG_RST_794"]
pub type Scuregrst794R = crate::BitReader;
#[doc = "Field `SCUREGRST794` writer - SCU_REG_RST_794"]
pub type Scuregrst794W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST798` reader - SCU_REG_RST_798"]
pub type Scuregrst798R = crate::BitReader;
#[doc = "Field `SCUREGRST798` writer - SCU_REG_RST_798"]
pub type Scuregrst798W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `SCUREGRST7C0` reader - SCU_REG_RST_7C0"]
pub type Scuregrst7c0R = crate::BitReader;
#[doc = "Field `SCUREGRST7C0` writer - SCU_REG_RST_7C0"]
pub type Scuregrst7c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST7C4` reader - SCU_REG_RST_7C4"]
pub type Scuregrst7c4R = crate::BitReader;
#[doc = "Field `SCUREGRST7C4` writer - SCU_REG_RST_7C4"]
pub type Scuregrst7c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST7C8` reader - SCU_REG_RST_7C8"]
pub type Scuregrst7c8R = crate::BitReader;
#[doc = "Field `SCUREGRST7C8` writer - SCU_REG_RST_7C8"]
pub type Scuregrst7c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST7CC` reader - SCU_REG_RST_7CC"]
pub type Scuregrst7ccR = crate::BitReader;
#[doc = "Field `SCUREGRST7CC` writer - SCU_REG_RST_7CC"]
pub type Scuregrst7ccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST7D0` reader - SCU_REG_RST_7D0"]
pub type Scuregrst7d0R = crate::BitReader;
#[doc = "Field `SCUREGRST7D0` writer - SCU_REG_RST_7D0"]
pub type Scuregrst7d0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST7D4` reader - SCU_REG_RST_7D4"]
pub type Scuregrst7d4R = crate::BitReader;
#[doc = "Field `SCUREGRST7D4` writer - SCU_REG_RST_7D4"]
pub type Scuregrst7d4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST7D8` reader - SCU_REG_RST_7D8"]
pub type Scuregrst7d8R = crate::BitReader;
#[doc = "Field `SCUREGRST7D8` writer - SCU_REG_RST_7D8"]
pub type Scuregrst7d8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_RST_780"]
    #[inline(always)]
    pub fn scuregrst780(&self) -> Scuregrst780R {
        Scuregrst780R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_784"]
    #[inline(always)]
    pub fn scuregrst784(&self) -> Scuregrst784R {
        Scuregrst784R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_REG_RST_788"]
    #[inline(always)]
    pub fn scuregrst788(&self) -> Scuregrst788R {
        Scuregrst788R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_REG_RST_78C"]
    #[inline(always)]
    pub fn scuregrst78c(&self) -> Scuregrst78cR {
        Scuregrst78cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_REG_RST_790"]
    #[inline(always)]
    pub fn scuregrst790(&self) -> Scuregrst790R {
        Scuregrst790R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_RST_794"]
    #[inline(always)]
    pub fn scuregrst794(&self) -> Scuregrst794R {
        Scuregrst794R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_RST_798"]
    #[inline(always)]
    pub fn scuregrst798(&self) -> Scuregrst798R {
        Scuregrst798R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - SCU_REG_RST_7C0"]
    #[inline(always)]
    pub fn scuregrst7c0(&self) -> Scuregrst7c0R {
        Scuregrst7c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_RST_7C4"]
    #[inline(always)]
    pub fn scuregrst7c4(&self) -> Scuregrst7c4R {
        Scuregrst7c4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_RST_7C8"]
    #[inline(always)]
    pub fn scuregrst7c8(&self) -> Scuregrst7c8R {
        Scuregrst7c8R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_REG_RST_7CC"]
    #[inline(always)]
    pub fn scuregrst7cc(&self) -> Scuregrst7ccR {
        Scuregrst7ccR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_REG_RST_7D0"]
    #[inline(always)]
    pub fn scuregrst7d0(&self) -> Scuregrst7d0R {
        Scuregrst7d0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_RST_7D4"]
    #[inline(always)]
    pub fn scuregrst7d4(&self) -> Scuregrst7d4R {
        Scuregrst7d4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SCU_REG_RST_7D8"]
    #[inline(always)]
    pub fn scuregrst7d8(&self) -> Scuregrst7d8R {
        Scuregrst7d8R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_RST_780"]
    #[inline(always)]
    pub fn scuregrst780(&mut self) -> Scuregrst780W<Scuf3cSpec> {
        Scuregrst780W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_784"]
    #[inline(always)]
    pub fn scuregrst784(&mut self) -> Scuregrst784W<Scuf3cSpec> {
        Scuregrst784W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_REG_RST_788"]
    #[inline(always)]
    pub fn scuregrst788(&mut self) -> Scuregrst788W<Scuf3cSpec> {
        Scuregrst788W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_REG_RST_78C"]
    #[inline(always)]
    pub fn scuregrst78c(&mut self) -> Scuregrst78cW<Scuf3cSpec> {
        Scuregrst78cW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_REG_RST_790"]
    #[inline(always)]
    pub fn scuregrst790(&mut self) -> Scuregrst790W<Scuf3cSpec> {
        Scuregrst790W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_RST_794"]
    #[inline(always)]
    pub fn scuregrst794(&mut self) -> Scuregrst794W<Scuf3cSpec> {
        Scuregrst794W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_RST_798"]
    #[inline(always)]
    pub fn scuregrst798(&mut self) -> Scuregrst798W<Scuf3cSpec> {
        Scuregrst798W::new(self, 6)
    }
    #[doc = "Bit 16 - SCU_REG_RST_7C0"]
    #[inline(always)]
    pub fn scuregrst7c0(&mut self) -> Scuregrst7c0W<Scuf3cSpec> {
        Scuregrst7c0W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_RST_7C4"]
    #[inline(always)]
    pub fn scuregrst7c4(&mut self) -> Scuregrst7c4W<Scuf3cSpec> {
        Scuregrst7c4W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_RST_7C8"]
    #[inline(always)]
    pub fn scuregrst7c8(&mut self) -> Scuregrst7c8W<Scuf3cSpec> {
        Scuregrst7c8W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_REG_RST_7CC"]
    #[inline(always)]
    pub fn scuregrst7cc(&mut self) -> Scuregrst7ccW<Scuf3cSpec> {
        Scuregrst7ccW::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_REG_RST_7D0"]
    #[inline(always)]
    pub fn scuregrst7d0(&mut self) -> Scuregrst7d0W<Scuf3cSpec> {
        Scuregrst7d0W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_RST_7D4"]
    #[inline(always)]
    pub fn scuregrst7d4(&mut self) -> Scuregrst7d4W<Scuf3cSpec> {
        Scuregrst7d4W::new(self, 21)
    }
    #[doc = "Bit 22 - SCU_REG_RST_7D8"]
    #[inline(always)]
    pub fn scuregrst7d8(&mut self) -> Scuregrst7d8W<Scuf3cSpec> {
        Scuregrst7d8W::new(self, 22)
    }
}
#[doc = "Reset Control 16 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf3cSpec;
impl crate::RegisterSpec for Scuf3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf3c::R`](R) reader structure"]
impl crate::Readable for Scuf3cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf3c::W`](W) writer structure"]
impl crate::Writable for Scuf3cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF3C to value 0"]
impl crate::Resettable for Scuf3cSpec {}
