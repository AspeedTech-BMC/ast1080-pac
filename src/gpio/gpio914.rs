#[doc = "Register `GPIO914` reader"]
pub type R = crate::R<Gpio914Spec>;
#[doc = "Register `GPIO914` writer"]
pub type W = crate::W<Gpio914Spec>;
#[doc = "Field `GPIO004ReadPrivilegeOfMaster` reader - GPIO004 Read Privilege of Master"]
pub type Gpio004readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO004ReadPrivilegeOfMaster` writer - GPIO004 Read Privilege of Master"]
pub type Gpio004readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO005ReadPrivilegeOfMaster` reader - GPIO005 Read Privilege of Master"]
pub type Gpio005readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO005ReadPrivilegeOfMaster` writer - GPIO005 Read Privilege of Master"]
pub type Gpio005readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO006ReadPrivilegeOfMaster` reader - GPIO006 Read Privilege of Master"]
pub type Gpio006readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO006ReadPrivilegeOfMaster` writer - GPIO006 Read Privilege of Master"]
pub type Gpio006readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO007ReadPrivilegeOfMaster` reader - GPIO007 Read Privilege of Master"]
pub type Gpio007readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO007ReadPrivilegeOfMaster` writer - GPIO007 Read Privilege of Master"]
pub type Gpio007readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO004 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio004read_privilege_of_master(&self) -> Gpio004readPrivilegeOfMasterR {
        Gpio004readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO005 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio005read_privilege_of_master(&self) -> Gpio005readPrivilegeOfMasterR {
        Gpio005readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO006 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio006read_privilege_of_master(&self) -> Gpio006readPrivilegeOfMasterR {
        Gpio006readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO007 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio007read_privilege_of_master(&self) -> Gpio007readPrivilegeOfMasterR {
        Gpio007readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO004 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio004read_privilege_of_master(
        &mut self,
    ) -> Gpio004readPrivilegeOfMasterW<Gpio914Spec> {
        Gpio004readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO005 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio005read_privilege_of_master(
        &mut self,
    ) -> Gpio005readPrivilegeOfMasterW<Gpio914Spec> {
        Gpio005readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO006 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio006read_privilege_of_master(
        &mut self,
    ) -> Gpio006readPrivilegeOfMasterW<Gpio914Spec> {
        Gpio006readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO007 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio007read_privilege_of_master(
        &mut self,
    ) -> Gpio007readPrivilegeOfMasterW<Gpio914Spec> {
        Gpio007readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio914::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio914::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio914Spec;
impl crate::RegisterSpec for Gpio914Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio914::R`](R) reader structure"]
impl crate::Readable for Gpio914Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio914::W`](W) writer structure"]
impl crate::Writable for Gpio914Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO914 to value 0xffff_ffff"]
impl crate::Resettable for Gpio914Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
