#[doc = "Register `DEV0C` reader"]
pub type R = crate::R<Dev0cSpec>;
#[doc = "Register `DEV0C` writer"]
pub type W = crate::W<Dev0cSpec>;
#[doc = "Field `BaseAddrOfDataBuf310` reader - Base address of data buffer\\[31:0\\]"]
pub type BaseAddrOfDataBuf310R = crate::FieldReader<u32>;
#[doc = "Field `BaseAddrOfDataBuf310` writer - Base address of data buffer\\[31:0\\]"]
pub type BaseAddrOfDataBuf310W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address of data buffer\\[31:0\\]"]
    #[inline(always)]
    pub fn base_addr_of_data_buf310(&self) -> BaseAddrOfDataBuf310R {
        BaseAddrOfDataBuf310R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address of data buffer\\[31:0\\]"]
    #[inline(always)]
    pub fn base_addr_of_data_buf310(&mut self) -> BaseAddrOfDataBuf310W<Dev0cSpec> {
        BaseAddrOfDataBuf310W::new(self, 0)
    }
}
#[doc = "Base Address of Endpoint 0 IN/OUT Data Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev0cSpec;
impl crate::RegisterSpec for Dev0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev0c::R`](R) reader structure"]
impl crate::Readable for Dev0cSpec {}
#[doc = "`write(|w| ..)` method takes [`dev0c::W`](W) writer structure"]
impl crate::Writable for Dev0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV0C to value 0"]
impl crate::Resettable for Dev0cSpec {}
